use std::{env, fs, io, path::Path};

use anyhow::{bail, Result};
use minutus_mruby_build_utils::MRubyManager;

fn check_command(cmd: &[&str]) {
    if std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .output()
        .is_err()
    {
        println!("cargo:warning={} command does not exist", cmd[1]);
        panic!("{} command does not exist", cmd[1]);
    }
}

fn extract_mruby_source_code() -> Result<()> {
    let workdir = env::var("OUT_DIR")?;
    let workdir = Path::new(&workdir);

    let archive_path = env::current_dir()?
        .join("mrubies")
        .join(format!("{}.tar.gz", mruby_version()));
    if !archive_path.exists() {
        println!("cargo:warning={} does not exist", archive_path.display());
        bail!("{archive_path:?} does not exist")
    }

    if workdir.join("mruby").exists() {
        return Ok(());
    }

    let tar_gz = fs::read(archive_path)?;
    let tar = {
        use bytes::Buf;
        flate2::read::GzDecoder::new(tar_gz.reader())
    };
    let mut archive = tar::Archive::new(tar);
    archive.unpack(workdir).unwrap();

    fs::rename(
        workdir.join(format!("mruby-{}", mruby_version())),
        workdir.join("mruby"),
    )?;

    Ok(())
}

fn build_on_doc_rs() -> Result<()> {
    extract_mruby_source_code()?;
    MRubyManager::new()
        .mruby_version(&mruby_version())
        .link(true)
        .download(false)
        .run();
    compile_bridge()?;

    println!("Finish build.rs");

    Ok(())
}

fn main() -> Result<()> {
    // docs.rs does not allow network access. So we need different settings.
    if std::env::var("DOCS_RS").is_ok() {
        build_on_doc_rs()?;
        return Ok(());
    }

    check_command(&["ruby", "-v"]);
    println!("cargo:rerun-if-changed=src/bridge");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR")?;
    let build_config_copy = Path::new(&out_dir).join("build_config.rb");
    fs::copy(
        env::current_dir()?.join("build_config.rb"),
        &build_config_copy,
    )?;

    let do_link = env::var("CARGO_FEATURE_LINK_MRUBY").is_ok();
    let do_download = match env::var("CARGO_FEATURE_MRUBY_DIR") {
        Err(_) => true,
        _ => match env::var("MINUTUS_MRUBY_DIR") {
            Ok(dir) if dir.trim().is_empty() => true,
            // copy_dir ok => No need to download.
            // copy_dir err => need to download.
            Ok(dir) => copy_to_mruby_outdir(&dir, &out_dir).is_err(),
            _ => true,
        },
    };

    MRubyManager::new()
        .mruby_version(&mruby_version())
        .link(do_link)
        .build_config(&build_config_copy)
        .download(do_download)
        .run();
    compile_bridge()?;

    println!("Finish build.rs");

    Ok(())
}

fn copy_to_mruby_outdir(
    local_dir: &str,
    out_dir: &str,
) -> fs_extra::error::Result<u64> {
    use fs_extra::dir::{copy as copy_dir, CopyOptions};

    let opts = CopyOptions::new().overwrite(true);
    println!("cargo:warning=local mruby dir: {local_dir}");

    let status = copy_dir(local_dir, out_dir, &opts).inspect_err(|e| {
        println!("cargo:warning=Failed to copy dir;\n outdir: {out_dir};\n Err: {e}")
    });
    let out_path = Path::new(out_dir);

    let dir_name = Path::new(local_dir)
        .file_name()
        .ok_or_else(|| io::Error::other("Failed to get local_dir.file_name"))?;

    println!("cargo:warning=out_dir: {out_dir}");

    fs::rename(out_path.join(dir_name), out_path.join("mruby")) //
        .inspect_err(|e| {
            println!("cargo:warning=Failed to rename to mruby;\n Err: {e}")
        })?;
    status
}

fn mruby_version() -> String {
    let default = "3.3.0";
    let supported_versions = &["3.1.0", "3.2.0", "3.3.0", "MASTER"];
    for version in supported_versions {
        if env::var(format!(
            "CARGO_FEATURE_MRUBY_{}",
            str::replace(version, ".", "_")
        ))
        .is_ok()
        {
            return version
                .to_lowercase()
                .to_string();
        }
    }
    default.to_string()
}

fn compile_bridge() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    // generate bridge.c
    let output = std::process::Command::new("ruby")
        .args(["all.rb"])
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
        bail!("Failed to execute command")
    }

    let existing_bridge = fs::read(out_dir.join("bridge.c"));
    let bridge_changed = existing_bridge
        .map(|existing_bridge| existing_bridge != output.stdout)
        .unwrap_or(true);
    if bridge_changed {
        fs::write(out_dir.join("bridge.c"), output.stdout)?;
    }

    // generate binding
    println!("Start generating binding");

    let mruby_include_path = Path::new(out_dir)
        .join("mruby")
        .join("include");
    println!(
        "include path: {}",
        mruby_include_path
            .to_str()
            .unwrap()
    );

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
        .clang_arg(format!(
            "-I{}",
            mruby_include_path
                .to_str()
                .unwrap()
        ))
        .header(
            out_dir
                .join("bridge.c")
                .to_string_lossy(),
        )
        .allowlist_type(allowlist_types.join("|"))
        .allowlist_function(allowlist_functions.join("|"))
        .layout_tests(false)
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
