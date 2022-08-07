use anyhow::Result;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    setup_mruby(&out_dir, &mruby_version()).unwrap();
    compile_bridge(&out_dir)?;

    if do_link() {
        println!("cargo:rustc-link-lib=mruby");
        println!("cargo:rustc-link-search={}/mruby/build/host/lib", out_dir);
    }

    Ok(())
}

fn setup_mruby(out_dir: &str, version: &str) -> std::io::Result<()> {
    Command::new("git")
        .current_dir(out_dir)
        .args(["clone", "--depth=1", "https://github.com/mruby/mruby.git"])
        .output()?;

    let mruby_dir = Path::new(out_dir).join("mruby");
    Command::new("git")
        .current_dir(mruby_dir.clone())
        .args(["checkout", version])
        .output()?;
    Command::new("rake")
        .current_dir(mruby_dir)
        .output()
        .expect("failed to build mruby");

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

    Command::new("ruby")
        .current_dir("bridge")
        .args(["all.rb"])
        .output()?;

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

    cc::Build::new()
        .file("src/bridge.c")
        .include(mruby_include_path)
        .compile("minutus_bridge");

    Ok(())
}
