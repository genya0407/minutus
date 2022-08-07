# mruby-polars   [![build](https://github.com/genya0407/mruby-polars/actions/workflows/ci.yml/badge.svg)](https://github.com/genya0407/mruby-polars/actions/workflows/ci.yml)
Polars class
## install by mrbgems
- add conf.gem line to `build_config.rb`

```ruby
MRuby::Build.new do |conf|

    # ... (snip) ...

    conf.gem :github => 'genya0407/mruby-polars'
end
```
## example
```ruby
p Polars.hi
#=> "hi!!"
t = Polars.new "hello"
p t.hello
#=> "hello"
p t.bye
#=> "hello bye"
```

## License
under the MIT License:
- see LICENSE file
