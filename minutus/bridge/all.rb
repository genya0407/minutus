require 'erb'

OUT="#{__dir__}/../src/bridge.c"

headers = <<~HEAD
#include "mruby.h"
#include "mruby/data.h"
#include "mruby/value.h"
#include "mruby/compile.h"
#include "mruby/class.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/hash.h"
#include "mruby/error.h"

// In order to use mrb_get_backtrace in mruby master (a0c02e0a6465ff9f37b7b2e4801081cef7c0e93c).
#if __has_include("mruby/internal.h")
# include "mruby/internal.h"
#endif

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
  exception
  symbol
  other
]
contents = files.map do |name|
  `ruby #{name}.rb`
end
all = [headers, *contents].join("\n")
File.write(OUT, all)
system("clang-format -i #{OUT}", exception: true)
