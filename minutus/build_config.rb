MRuby::Build.new do |conf|
  conf.toolchain
  conf.gembox 'default'
  conf.enable_bintest
  conf.enable_test
end