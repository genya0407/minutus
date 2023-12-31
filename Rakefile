task :run_all do
  cargo_clean = ENV["CLEAN"] ? '&& cargo clean' : ''
  rake_clean = ENV["CLEAN"] ? 'clean' : ''

  sh "cd minutus-mrbgem-template #{cargo_clean}"
  sh "cd minutus-mrbgem-template && ./test.sh"
  sh "cd minutus-mrbgem-template && ./test_with_dependency.sh"
end
