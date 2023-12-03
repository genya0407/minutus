fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_config.rb");

    minutus::MRubyManager::new()
        .mruby_version("3.2.0")
        .build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .run();
}
