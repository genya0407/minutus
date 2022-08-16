MRuby::Build.new do |conf|
  conf.toolchain
  conf.gembox 'full-core'
  conf.enable_bintest
  conf.enable_test
end