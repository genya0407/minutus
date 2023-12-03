MRuby::Build.new do |conf|
  conf.toolchain
  conf.gembox 'default'
  conf.gem core: 'mruby-error'
  conf.enable_bintest
  conf.enable_test
end