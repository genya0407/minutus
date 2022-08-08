require 'erb'

into_mrb = %w[
  false
  nil
  true
]

value_c = ERB.new(<<~TEMPLATE).result
<% into_mrb.each do |type| %>
minu_value minu_<%= type %>_value()
{
  return mrb_<%= type %>_value();
}
<% end %>

// _func postfix because of symbol duplication
minu_float minu_float_func(minu_value v) { return mrb_float(v); }

// _func postfix because of symbol duplication
minu_int minu_fixnum_func(minu_value v) { return mrb_fixnum(v); }

minu_value minu_fixnum_value(minu_int v)
{
  return mrb_fixnum_value(v);
}

minu_value minu_float_value(minu_state *minu, minu_float v)
{
  return mrb_float_value(minu, v);
}

minu_value minu_obj_value(void * v)
{
  return mrb_obj_value(v);
}
TEMPLATE

puts value_c