require 'erb'

OUT="#{__dir__}/../src/bridge.c"

File.write(OUT, <<~HEAD)
#include "mruby.h"
#include "mruby/data.h"
#include "mruby/value.h"
#include "mruby/compile.h"
#include "mruby/class.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/hash.h"
#include "mruby/error.h"
HEAD

files = %w[
  types
  predicates
  values
  data
  methods
  string
  array
  hash
  gc
  other
]
files.each do |name|
  system("ruby #{name}.rb >> #{OUT}", exception: true)
end

system("clang-format -i #{OUT}", exception: true)
