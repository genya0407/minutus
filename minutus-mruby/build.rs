use anyhow::{anyhow, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    download_mruby(&out_dir, &mruby_version()).unwrap();
    compile_bridge(&out_dir)?;

    if do_link() {
        build_mruby(&out_dir);
        println!("cargo:rustc-link-lib=mruby");
        println!("cargo:rustc-link-search={}/mruby/build/host/lib", out_dir);
    }

    println!("Finish build.rs");

    Ok(())
}

fn run_command(current_dir: &str, cmd: &[&str]) -> Result<()> {
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

fn build_mruby(out_dir: &str) {
    run_command(
        Path::new(out_dir).join("mruby").to_str().unwrap(),
        &["rake"],
    )
    .unwrap();
}

fn download_mruby(out_dir: &str, version: &str) -> Result<()> {
    let url = if version == "master" {
        String::from("https://github.com/mruby/mruby/archive/refs/heads/master.tar.gz")
    } else {
        format!(
            "https://github.com/mruby/mruby/archive/refs/tags/{}.tar.gz",
            version
        )
    };
    run_command(out_dir, &["wget", &url, "-O", "mruby.tar.gz"])?;
    run_command(out_dir, &["tar", "zxf", "mruby.tar.gz"])?;
    run_command(out_dir, &["mv", &format!("mruby-{}", version), "mruby"])?;

    Ok(())
}

fn do_link() -> bool {
    env::var("CARGO_FEATURE_LINK_MRUBY").is_ok()
}

fn mruby_version() -> String {
    let supported_versions = &["3.1.0", "2.1.2", "master"];
    for version in supported_versions.into_iter() {
        if env::var(format!(
            "CARGO_FEATURE_MRUBY_{}",
            str::replace(version, ".", "_")
        ))
        .is_ok()
        {
            return version.to_string();
        }
    }
    panic!("No mruby version is specified")
}

fn compile_bridge(out_dir: &str) -> Result<()> {
    let mruby_include_path = Path::new(out_dir).join("mruby/include");
    let out_path = Path::new(out_dir).join("mruby.rs");

    run_command("bridge", &["ruby", "all.rb"])?;

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
        .header("src/bridge.c")
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
        .file("src/bridge.c")
        .include(mruby_include_path)
        .compile("minutus_bridge");

    println!("Finish compiling binding");

    Ok(())
}
