task :run_all do
  cargo_clean = ENV["CLEAN"] ? '&& cargo clean' : ''
  rake_clean = ENV["CLEAN"] ? 'clean' : ''

  sh "cd examples/plain-mruby #{cargo_clean} && cargo run"
  sh "cd examples/custom-mruby #{cargo_clean} && cargo run && cargo test"
  sh "cd examples/custom-mruby-3_1_0 #{cargo_clean} && cargo run && cargo test"
  sh "cd examples/mruby-polars && rake #{rake_clean} test"
  sh "cd minutus-mrbgem-template #{cargo_clean}"
  sh "cd minutus-mrbgem-template && ./test.sh"
  sh "cd minutus-mrbgem-template && ./test_with_dependency.sh"
end
