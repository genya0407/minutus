require 'erb'

types = %w[
  array
  class
  data
  exception
  false
  fixnum
  float
  hash
  module
  nil
  object
  range
  string
  true
]

predicate_c = ERB.new(<<~TEMPLATE).result
<% types.each do |type| %>
minu_bool minu_<%= type %>_p(minu_value v)
{
  return mrb_<%= type %>_p(v);
}
<% end %>
TEMPLATE

puts predicate_c