fn main() {
    println!("cargo:rerun-if-changed=build_config.rb");

    let out_dir_str = std::env::var("OUT_DIR").unwrap();
    let base_dir = std::path::Path::new(&out_dir_str);

    let builder = minutus::MRubyBuilder {
        base_dir: &base_dir,
        mruby_version: String::from("3.1.0"),
    };
    builder.download_mruby().unwrap();
    builder
        .link_mruby_with_build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .unwrap();
}
