MRuby::Gem::Specification.new('mruby-polars') do |spec|
  
  spec.license = 'MIT'
  spec.authors = 'Yusuke Sangenya'

  sh "cd #{__dir__} && cargo build --release"
  spec.linker.libraries << 'mruby_polars'
  spec.linker.library_paths << "#{__dir__}/target/release"

  if RbConfig::CONFIG['host_os'].downcase.include?('linux')
    spec.linker.libraries << 'dl'
    spec.linker.flags << '-pthread'
  end

  if RbConfig::CONFIG['host_os'].downcase.include?('darwin')
    spec.linker.flags << '-framework CoreFoundation'
  end
end
