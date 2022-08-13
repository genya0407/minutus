use anyhow::{anyhow, Result};
use std::env;
use std::path::Path;

use minutus_mruby_build_utils::MRubyManager;

fn check_command(cmd: &[&str]) {
    if std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .output()
        .is_err()
    {
        println!("cargo:warning=ruby command does not exist");
        panic!("ruby command does not exist");
    }
}

fn main() -> Result<()> {
    check_command(&["ruby", "-v"]);
    check_command(&["clang-format", "--help"]);

    println!("cargo:rerun-if-changed=src/bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let do_link = env::var("CARGO_FEATURE_LINK_MRUBY").is_ok();
    MRubyManager::new()
        .mruby_version(&mruby_version())
        .link(do_link)
        .run();
    compile_bridge()?;

    println!("Finish build.rs");

    Ok(())
}

fn mruby_version() -> String {
    let default = "3.1.0";
    let supported_versions = &["3.1.0", "MASTER"];
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

fn compile_bridge() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    // generate bridge.c
    let output = std::process::Command::new("ruby")
        .args(&["all.rb"])
        .current_dir(Path::new("src").join("bridge"))
        .output();
    let output = match output {
        Ok(o) => o,
        Err(e) => {
            println!("cargo:warning={}", e);
            panic!("{}", e);
        }
    };
    if !output.status.success() {
        eprintln!("{}", String::from_utf8(output.stderr)?);
        return Err(anyhow!("Failed to execute command"));
    }
    std::fs::write(out_dir.join("bridge.c"), output.stdout)?;

    // generate binding
    println!("Start generating binding");

    let mruby_include_path = Path::new(out_dir).join("mruby").join("include");
    println!("include path: {}", mruby_include_path.to_str().unwrap());

    let out_path = Path::new(out_dir).join("mruby.rs");
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
        .header(out_dir.join("bridge.c").to_string_lossy())
        .allowlist_type(allowlist_types.join("|"))
        .allowlist_function(allowlist_functions.join("|"))
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()?;
    bindings.write_to_file(out_path)?;

    println!("Finish generating binding");

    // Compile
    println!("Start compiling binding");

    cc::Build::new()
        .file(out_dir.join("bridge.c"))
        .include(mruby_include_path)
        .compile("minutus_bridge");

    println!("Finish compiling binding");

    Ok(())
}
