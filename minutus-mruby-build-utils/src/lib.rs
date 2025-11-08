use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

mod mruby_dir;

/// Helper for building and linking libmruby.
#[derive(Debug)]
pub struct MRubyManager {
    workdir: Option<PathBuf>,
    mruby_version: Option<String>,
    do_link: bool,
    build_config: Option<PathBuf>,
    do_download: bool,
    copy_mruby_from: Option<PathBuf>,
}

impl Default for MRubyManager {
    fn default() -> Self {
        Self {
            workdir: None,
            mruby_version: None,
            do_link: true,
            build_config: None,
            do_download: true,
            copy_mruby_from: None,
        }
    }
}

impl MRubyManager {
    /// Construct a new instance of a blank set of configuration.
    /// This builder is finished with the [run][`MRubyManager::run()`] function.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set workdir. The default is `"OUT_DIR"` environment variable.
    pub fn workdir(mut self, path: &Path) -> Self {
        self.workdir = Some(path.to_path_buf());
        self
    }

    /// Set mruby version.
    pub fn mruby_version(mut self, mruby_version: &str) -> Self {
        self.mruby_version = Some(mruby_version.to_string());
        self
    }

    /// Set custom `build_config.rb`. If not set, the builder uses mruby's default
    /// config.
    pub fn build_config(mut self, build_config: &Path) -> Self {
        self.build_config = Some(build_config.to_path_buf());
        self
    }

    /// Set custom `mruby_dir`
    ///
    /// # Example
    ///
    /// ```ignore
    /// MRubyManager::new().copy_mruby_from("/path/to/mruby-src-dir").run()
    /// ```
    pub fn copy_mruby_from<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.copy_mruby_from = Some(path.into());
        self
    }

    /// Whether the builder should build/link `libmruby.a` or not. The default is
    /// `true`.
    ///
    /// If set to `false`, builder does not build nor link libmruby. So you have
    /// to do it by yourself.
    ///
    /// If you embed mruby into your Rust project, this should be `true`.
    pub fn link(mut self, doit: bool) -> Self {
        self.do_link = doit;
        self
    }

    /// Whether the builder should internally download mruby source code or not.
    /// The default is `true`.
    ///
    /// If set to `false` you have to place `$OUT_DIR/mruby` by yourself.
    pub fn download(mut self, doit: bool) -> Self {
        self.do_download = doit;
        self
    }

    /// Run the task.
    pub fn run(self) {
        let workdir = self.workdir.unwrap_or_else(|| {
            std::env::var("OUT_DIR")
                .expect(r#"Could not fetch "OUT_DIR" environment variable."#)
                .into()
        });

        let build_config = self
            .build_config
            .unwrap_or_else(|| "default".into()); // see: https://github.com/mruby/mruby/blob/3.2.0/doc/guides/compile.md#build

        if self.do_download {
            let mruby_version = self
                .mruby_version
                .expect("mruby_version is not set.");

            download_mruby(&workdir, &mruby_version);
        }

        if let Some(src_dir) = self.copy_mruby_from {
            mruby_dir::copy_to_mruby_dir(&src_dir, &workdir).unwrap_or_else(|_| {
        panic!("Failed to copy dir. src: {src_dir:?}, target: {workdir:?}/mruby")
      });
        }

        build_mruby(&workdir, &build_config);

        if self.do_link {
            link_mruby(&workdir);
        }
    }
}

fn build_mruby(workdir: &Path, path: &Path) {
    let rake = if cfg!(windows) { "rake.bat" } else { "rake" };

    let c = &[
        rake,
        "all",
        &format!("MRUBY_CONFIG={}", path.to_string_lossy()),
    ];
    run_command(&workdir.join("mruby"), c).unwrap();
}

fn link_mruby(workdir: &Path) {
    let mrb_cfg_bin = workdir
        // On Windows, you don't need to manually change path separators "/" to "\\",
        // because Rust std handles them automatically.
        //
        // Note: Modern Unix-like are compatible with the POSIX path separator `/`.
        .join("mruby/bin/mruby-config");

    #[allow(unreachable_patterns)]
    let mruby_config = match mrb_cfg_bin {
        #[cfg(windows)]
        p => Some(p.with_extension("bat")).filter(|x| x.exists()),
        #[cfg(not(windows))]
        p if p.exists() => Some(p),
        _ => None,
    }
    .expect(r#"The `mruby-config` executable file does not exist!"#);

    let ldflags_before_libs = run_command(
        workdir,
        &[mruby_config.to_str().unwrap(), "--ldflags-before-libs"],
    )
    .unwrap();
    let ldflags =
        run_command(workdir, &[mruby_config.to_str().unwrap(), "--ldflags"])
            .unwrap();
    let libs =
        run_command(workdir, &[mruby_config.to_str().unwrap(), "--libs"]).unwrap();
    println!(
        "cargo:rustc-flags={} {} {}",
        ldflags_before_libs.trim(),
        ldflags.trim(),
        libs.trim()
    );
}

/// Downloads mruby source code from github.
pub fn download_mruby(workdir: &Path, mruby_version: &str) {
    if workdir.join("mruby").exists() {
        return;
    }

    let url = match mruby_version {
        "master" => {
            "https://github.com/mruby/mruby/archive/refs/heads/master.tar.gz".into()
        }
        _ => {
            format!(
        "https://github.com/mruby/mruby/archive/refs/tags/{mruby_version}.tar.gz"
      )
        }
    };

    let resp = reqwest::blocking::get(url).unwrap();
    let tar_gz = resp.bytes().unwrap();
    let tar = {
        use bytes::Buf;
        flate2::read::GzDecoder::new(tar_gz.reader())
    };
    let mut archive = tar::Archive::new(tar);
    archive.unpack(workdir).unwrap();

    std::fs::rename(
        workdir.join(format!("mruby-{mruby_version}")),
        workdir.join("mruby"),
    )
    .unwrap();
}

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<String> {
    println!("Start: {cmd:?}");

    let output = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(current_dir)
        .output()?;

    let [stdout, stderr] = [&output.stdout, &output.stderr] //
        .map(|buf| String::from_utf8_lossy(buf));

    if !output.status.success() {
        bail!("Executing {cmd:?} failed!\n stdout: {stdout}\n stderr: {stderr}")
    }

    Ok(stdout.into_owned())
}
