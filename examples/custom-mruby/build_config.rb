MRuby::Build.new do |conf|
  conf.toolchain :clang
  
  conf.gembox 'default'
  conf.gem github: 'mattn/mruby-json'
end
