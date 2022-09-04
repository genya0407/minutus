MRuby::Build.new do |conf|
  toolchain :gcc
  
  conf.gembox 'default'
  conf.gem github: 'matsumoto-r/mruby-mrbgem-template'
end
