use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

/// Helper for building and linking libmruby.
pub struct MRubyManager {
    workdir: Option<PathBuf>,
    mruby_version: Option<String>,
    do_link: bool,
    build_config: Option<PathBuf>,
    do_download: bool,
}

impl MRubyManager {
    /// Construct a new instance of a blank set of configuration.
    /// This builder is finished with the [run][`MRubyManager::run()`] function.
    pub fn new() -> Self {
        Self {
            workdir: None,
            mruby_version: None,
            do_link: true,
            build_config: None,
            do_download: true,
        }
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

    /// Set custom `build_config.rb`. If not set, the builder uses mruby's default config.
    pub fn build_config(mut self, build_config: &Path) -> Self {
        self.build_config = Some(build_config.to_path_buf());
        self
    }

    /// Whether the builder should build/link `libmruby.a` or not. The default is `true`.
    ///
    /// If set to `false`, builder does not build nor link libmruby. So you have to do it by yourself.
    ///
    /// If you embed mruby into your Rust project, this should be `true`.
    pub fn link(mut self, doit: bool) -> Self {
        self.do_link = doit;
        self
    }

    /// Whether the builder should internally download mruby source code or not. The default is `true`.
    ///
    /// If set to `false` you have to place `$OUT_DIR/mruby` by yourself.
    pub fn download(mut self, doit: bool) -> Self {
        self.do_download = doit;
        self
    }

    /// Run the task.
    pub fn run(self) {
        let workdir = self.workdir.unwrap_or_else(|| {
            let out_dir = std::env::var("OUT_DIR")
                .expect("Could not fetch \"OUT_DIR\" environment variable.");
            Path::new(&out_dir).to_path_buf()
        });
        let mruby_version = self
            .mruby_version
            .map(String::from)
            .expect("mruby_version is not set.");
        let build_config = self
            .build_config
            .unwrap_or(Path::new("default").to_path_buf()); // see: https://github.com/mruby/mruby/blob/3.2.0/doc/guides/compile.md#build

        if self.do_download {
            download_mruby(&workdir, &mruby_version);
        }
        build_mruby(&workdir, &build_config);

        if self.do_link {
            link_mruby(&workdir);
        }
    }
}

fn build_mruby(workdir: &Path, path: &Path) {
    let c = &[
        "rake",
        "all",
        &format!("MRUBY_CONFIG={}", path.to_string_lossy()),
    ];
    run_command(&workdir.join("mruby"), c).unwrap();
}

fn link_mruby(workdir: &Path) {
    let mruby_config = workdir.join("mruby").join("bin").join("mruby-config");
    let ldflags_before_libs = run_command(
        workdir,
        &[mruby_config.to_str().unwrap(), "--ldflags-before-libs"],
    )
    .unwrap();
    let ldflags = run_command(workdir, &[mruby_config.to_str().unwrap(), "--ldflags"]).unwrap();
    let libs = run_command(workdir, &[mruby_config.to_str().unwrap(), "--libs"]).unwrap();
    println!(
        "cargo:rustc-flags={} {} {}",
        ldflags_before_libs.trim(),
        ldflags.trim(),
        libs.trim()
    );

    // For build on environments where `-Wl,--as-needed` is the default.
    let re = regex::Regex::new(r"as-needed").unwrap();
    let as_needed_supported = re.is_match(&run_command(Path::new("."), &["ld", "--help"]).unwrap());
    if as_needed_supported {
        println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    }
    println!("cargo:rustc-link-lib=mruby");
}

/// Downloads mruby source code from github.
pub fn download_mruby(workdir: &Path, mruby_version: &str) {
    if workdir.join("mruby").exists() {
        return;
    }

    let url = if mruby_version == "master" {
        String::from("https://github.com/mruby/mruby/archive/refs/heads/master.tar.gz")
    } else {
        format!(
            "https://github.com/mruby/mruby/archive/refs/tags/{}.tar.gz",
            mruby_version
        )
    };

    let resp = reqwest::blocking::get(url).unwrap();
    let tar_gz = resp.bytes().unwrap();
    let tar = {
        use bytes::Buf;
        flate2::read::GzDecoder::new(tar_gz.reader())
    };
    let mut archive = tar::Archive::new(tar);
    archive.unpack(&workdir).unwrap();

    std::fs::rename(
        workdir.join(format!("mruby-{}", mruby_version)),
        workdir.join("mruby"),
    )
    .unwrap();
}

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<String> {
    println!("Start: {:?}", cmd);

    let output = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(current_dir)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(format!(
            "Executing {:?} failed: {}, {}",
            cmd,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}
