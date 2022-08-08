use anyhow::{anyhow, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir_str = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir_str);

    download_mruby(&out_dir, &mruby_version()).unwrap();
    compile_bridge(&out_dir)?;

    if do_link() {
        build_mruby(&out_dir)?;
        let search_dir = out_dir.join("mruby").join("build").join("host").join("lib");
        ls_dir(&search_dir)?;
        println!("cargo:rustc-link-lib=mruby");
        println!("cargo:rustc-link-search={}", search_dir.to_string_lossy());
    }

    println!("Finish build.rs");

    Ok(())
}

fn run_command(current_dir: &Path, cmd: &[&str]) -> Result<()> {
    println!("Start: {:?}", cmd);

    let status = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", &cmd.join(" ")])
            .current_dir(current_dir)
            .status()?
    } else {
        std::process::Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(current_dir)
            .status()?
    };

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(format!("Executing {:?} failed", cmd)))
    }
}

fn build_mruby(out_dir: &Path) -> Result<()> {
    run_command(&out_dir.join("mruby"), &["rake"])?;
    if cfg!(target_os = "windows") {
        let lib_dir = out_dir.join("mruby").join("build").join("host").join("lib");
        std::fs::rename(lib_dir.join("libmruby.a"), lib_dir.join("mruby.lib"))?;
    }
    Ok(())
}

fn download_mruby(out_dir: &Path, version: &str) -> Result<()> {
    for d in &["mruby", &format!("mruby-{}", version)] {
        let p = out_dir.join(d);
        println!("Start removing {}", p.to_str().unwrap());
        if p.exists() {
            std::fs::remove_dir_all(p.clone())?;
            println!("Finished removing {}", p.to_str().unwrap());
        } else {
            println!("Skip removing {}", p.to_str().unwrap());
        }
    }

    let url = if version == "master" {
        String::from("https://github.com/mruby/mruby/archive/refs/heads/master.tar.gz")
    } else {
        format!(
            "https://github.com/mruby/mruby/archive/refs/tags/{}.tar.gz",
            version
        )
    };

    let resp = reqwest::blocking::get(url)?;
    let tar_gz = resp.bytes()?;
    let tar = {
        use bytes::Buf;
        flate2::read::GzDecoder::new(tar_gz.reader())
    };
    let mut archive = tar::Archive::new(tar);
    archive.unpack(out_dir)?;

    std::fs::rename(
        out_dir.join(format!("mruby-{}", version)),
        out_dir.join("mruby"),
    )?;

    Ok(())
}

fn do_link() -> bool {
    env::var("CARGO_FEATURE_LINK_MRUBY").is_ok()
}

fn mruby_version() -> String {
    let default = "3.1.0";
    let supported_versions = &["3.1.0", "2.1.2", "MASTER"];
    for version in supported_versions.into_iter() {
        if env::var(format!(
            "CARGO_FEATURE_MRUBY_{}",
            str::replace(version, ".", "_")
        ))
        .is_ok()
        {
            return version.to_lowercase().to_string();
        }
    }
    return default.to_string();
}

fn compile_bridge(out_dir: &Path) -> Result<()> {
    let mruby_include_path = Path::new(out_dir).join("mruby").join("include");
    let out_path = Path::new(out_dir).join("mruby.rs");

    run_command(&Path::new("bridge"), &["ruby", "all.rb"])?;

    let allowlist_types = &[
        "minu_.*",
        "RClass",
        "RObject",
        "RBasic",
        "RData",
        "RString",
        "RInteger",
        "RFloat",
        "RRational",
        "RComplex",
        "RArray",
        "RHash",
        "RRange",
        "RProc",
        "RException",
    ];
    let allowlist_functions = &["minu_.*", "mrb_raise", "mrb_get_args"];

    println!("Start generating binding");

    println!("include path: {}", mruby_include_path.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", mruby_include_path.to_str().unwrap()))
        .header(Path::new("src").join("bridge.c").to_string_lossy())
        .allowlist_type(allowlist_types.join("|"))
        .allowlist_function(allowlist_functions.join("|"))
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()?;
    bindings.write_to_file(out_path)?;

    println!("Finish generating binding");

    println!("Start compiling binding");

    cc::Build::new()
        .file(Path::new("src").join("bridge.c"))
        .include(mruby_include_path)
        .compile("minutus_bridge");

    println!("Finish compiling binding");

    Ok(())
}

#[allow(dead_code)]
fn ls_dir(dir: &Path) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("{:?}", path);
    }
    Ok(())
}
