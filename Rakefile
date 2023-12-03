task :run_all do
  cargo_clean = ENV["CLEAN"] ? '&& cargo clean' : ''
  rake_clean = ENV["CLEAN"] ? 'clean' : ''

  sh "rm -rf tmp_examples_*"

  mruby_version = ENV.fetch("TEST_TARGET_MRUBY_VERSION").gsub('.', '_')
  sh "rm -rf tmp_examples_#{mruby_version} && cp -r examples tmp_examples_#{mruby_version}"
  Dir.glob("tmp_examples_#{mruby_version}/**/Cargo.toml").each do |cargo_toml|
    File.write(cargo_toml, File.read(cargo_toml).gsub('mruby_3_2_0', "mruby_#{mruby_version}"))
  end
  sh "cd tmp_examples_#{mruby_version}/plain-mruby #{cargo_clean} && cargo run"
  sh "cd tmp_examples_#{mruby_version}/custom-mruby #{cargo_clean} && cargo run"
  sh "cd tmp_examples_#{mruby_version}/mruby-polars && MRUBY_VERSION=#{mruby_version.gsub('_', '.')} rake #{rake_clean} test"
  sh "rm -rf tmp_examples_#{mruby_version}"

  sh "cd minutus-mrbgem-template #{cargo_clean}"
  sh "cd minutus-mrbgem-template && ./test.sh"
  sh "cd minutus-mrbgem-template && ./test_with_dependency.sh"
end
