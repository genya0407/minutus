use anyhow::{anyhow, Result};
use std::path::Path;

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<()> {
    println!("Start: {:?}", cmd);

    let status = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(current_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(format!("Executing {:?} failed", cmd)))
    }
}

pub struct MRubyBuilder<'a> {
    pub base_dir: &'a Path,
    pub mruby_version: String,
}

impl<'a> MRubyBuilder<'a> {
    pub fn link_mruby(&self) -> Result<()> {
        self.internal_link_mruby(None)
    }

    pub fn link_mruby_with_build_config(&self, build_config_path: &Path) -> Result<()> {
        self.internal_link_mruby(Some(build_config_path))
    }

    fn internal_link_mruby(&self, build_config_path: Option<&Path>) -> Result<()> {
        let search_dir = self
            .base_dir
            .join("mruby")
            .join("build")
            .join("host")
            .join("lib");
        let cmd = if let Some(path) = build_config_path {
            format!("rake clean all MRUBY_CONFIG={}", path.to_string_lossy())
        } else {
            String::from("rake clean all")
        };
        run_command(&self.base_dir.join("mruby"), &["sh", "-c", &cmd])?;
        println!("cargo:rustc-link-lib=mruby");
        println!("cargo:rustc-link-search={}", search_dir.to_string_lossy());
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
