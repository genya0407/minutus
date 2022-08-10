fn main() {
    println!("cargo:rerun-if-changed=build_config.rb");

    let builder = minutus::MRubyBuilder {
        base_dir: &std::env::current_dir().unwrap(),
        mruby_version: String::from("3.1.0"),
    };
    builder.download_mruby().unwrap();
    builder
        .link_mruby_with_build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .unwrap();
}
