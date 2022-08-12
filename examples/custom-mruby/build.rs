use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=bulid.rs");
    println!("cargo:rerun-if-changed=build_config.rb");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let builder = minutus::MRubyBuilder {
        base_dir: &out_dir,
        mruby_version: String::from("3.1.0"),
    };
    builder.download_mruby().unwrap();
    builder
        .link_mruby_with_build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .unwrap();
}
