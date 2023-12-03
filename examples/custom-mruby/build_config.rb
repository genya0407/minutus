MRuby::Build.new do |conf|
  toolchain
  conf.gembox 'default'
  conf.gem github: 'mattn/mruby-json', branch: 'master'
end
