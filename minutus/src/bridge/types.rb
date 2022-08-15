require 'erb'

types = %w[
  state
  aspec
  value
  int
  float
  bool
  sym
]

type_c = ERB.new(<<~TEMPLATE).result
<% types.each do |type| %>
typedef mrb_<%= type %> minu_<%= type %> ;
<% end %>

typedef mrb_func_t minu_func_t;
TEMPLATE

puts type_c