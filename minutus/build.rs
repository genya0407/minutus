use anyhow::{anyhow, Result};
use std::env;
use std::path::Path;

use minutus_mruby_build_utils::MRubyBuilder;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir_str = env::var("OUT_DIR")?;
    let base_dir = Path::new(&out_dir_str);

    let builder = MRubyBuilder {
        base_dir: base_dir,
        mruby_version: mruby_version(),
    };

    builder.download_mruby()?;
    compile_bridge(&base_dir)?;

    if env::var("CARGO_FEATURE_LINK_MRUBY").is_ok() {
        builder.link_mruby()?
    }

    println!("Finish build.rs");

    Ok(())
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

    let status = std::process::Command::new("ruby")
        .args(&["all.rb"])
        .current_dir("bridge")
        .status()?;
    if !status.success() {
        return Err(anyhow!("Failed to execute command"));
    }

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
