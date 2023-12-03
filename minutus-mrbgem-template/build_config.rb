MRuby::Build.new do |conf|
  toolchain :gcc
  
  conf.gembox 'default'
  conf.gem github: 'matsumoto-r/mruby-mrbgem-template'
  conf.gem core: 'mruby-error'
end
