use anyhow::{anyhow, Result};
use std::path::Path;

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<String> {
    println!("Start: {:?}", cmd);

    let output = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(current_dir)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow!(format!("Executing {:?} failed", cmd)))
    }
}

pub struct MRubyBuilder<'a> {
    base_dir: &'a Path,
    mruby_version: String,
}

impl<'a> MRubyBuilder<'a> {
    pub fn new(base_dir: &'a Path, mruby_version: String) -> Self {
        Self {
            base_dir,
            mruby_version,
        }
    }

    pub fn link_mruby(&self) -> Result<()> {
        self.internal_link_mruby(None)
    }

    pub fn link_mruby_with_build_config(&self, build_config_path: &Path) -> Result<()> {
        self.internal_link_mruby(Some(build_config_path))
    }

    fn internal_link_mruby(&self, build_config_path: Option<&Path>) -> Result<()> {
        let mruby_config = self.base_dir.join("mruby").join("bin").join("mruby-config");
        if let Some(path) = build_config_path {
            let c = &[
                "rake",
                "all",
                &format!("MRUBY_CONFIG={}", path.to_string_lossy()),
            ];
            run_command(&self.base_dir.join("mruby"), c)?;
        } else {
            let c = &["rake", "all"];
            run_command(&self.base_dir.join("mruby"), c)?;
        };

        let ldflags_before_libs = run_command(
            self.base_dir,
            &[mruby_config.to_str().unwrap(), "--ldflags-before-libs"],
        )?;
        let ldflags = run_command(
            self.base_dir,
            &[mruby_config.to_str().unwrap(), "--ldflags"],
        )?;
        let libs = run_command(self.base_dir, &[mruby_config.to_str().unwrap(), "--libs"])?;
        println!(
            "cargo:rustc-flags={} {} {}",
            ldflags_before_libs.trim(),
            ldflags.trim(),
            libs.trim()
        );

        // For build on environments where `-Wl,--as-needed` is the default.
        if cc::Build::new().is_flag_supported("-Wl,--no-as-needed")? {
            println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
            println!("cargo:rustc-link-arg=-lmruby");
        }

        Ok(())
    }

    pub fn download_mruby(&self) -> Result<()> {
        if self.base_dir.join("mruby").exists() {
            return Ok(());
        }

        let url = if self.mruby_version == "master" {
            String::from("https://github.com/mruby/mruby/archive/refs/heads/master.tar.gz")
        } else {
            format!(
                "https://github.com/mruby/mruby/archive/refs/tags/{}.tar.gz",
                self.mruby_version
            )
        };

        let resp = reqwest::blocking::get(url)?;
        let tar_gz = resp.bytes()?;
        let tar = {
            use bytes::Buf;
            flate2::read::GzDecoder::new(tar_gz.reader())
        };
        let mut archive = tar::Archive::new(tar);
        archive.unpack(&self.base_dir)?;

        std::fs::rename(
            self.base_dir.join(format!("mruby-{}", self.mruby_version)),
            self.base_dir.join("mruby"),
        )?;

        Ok(())
    }
}
