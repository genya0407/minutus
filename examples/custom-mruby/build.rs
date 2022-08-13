fn main() {
    println!("cargo:rerun-if-changed=bulid.rs");
    println!("cargo:rerun-if-changed=build_config.rb");
    let builder = minutus::MRubyManager::new()
        .build_config(&std::env::current_dir().unwrap().join("build_config.rb"))
        .mruby_version("3.1.0")
        .run();
}
