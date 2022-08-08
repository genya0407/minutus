use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build_config.rb");

    let builder = minutus::MRubyBuilder {
        base_dir: Path::new("."),
        mruby_version: String::from("2.1.2"),
    };
    builder.download_mruby().unwrap();
    builder
        .link_mruby_with_build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .unwrap();
}
