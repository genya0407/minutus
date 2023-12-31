MRuby::Build.new do |conf|
  toolchain :gcc
  
  conf.gembox 'full-core'
  conf.gem github: 'matsumoto-r/mruby-mrbgem-template'
end
