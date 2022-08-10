MRuby::Gem::Specification.new('mruby-polars') do |spec|
  
  spec.license = 'MIT'
  spec.authors = 'Yusuke Sangenya'

  system("cd #{__dir__} && cargo build --release", exception: true)
  spec.linker.libraries << 'mruby_polars'
  spec.linker.library_paths << "#{__dir__}/target/release"
  if RbConfig::CONFIG['host_os'].downcase.include?('darwin')
    spec.linker.flags << '-framework CoreFoundation'
  end
end
