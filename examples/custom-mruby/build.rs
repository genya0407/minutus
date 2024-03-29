fn main() {
    println!("cargo:rerun-if-changed=bulid.rs");
    println!("cargo:rerun-if-changed=build_config.rb");
    minutus::MRubyManager::new()
        .build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .mruby_version("3.2.0")
        .run();
}
