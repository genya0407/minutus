puts <<~CODE
minu_sym minu_intern_cstr(minu_state *mrb, const char* s) {
  return mrb_intern_cstr(mrb, s);
}

minu_value minu_symbol_value(mrb_sym i) {
  return mrb_symbol_value(i);
}

minu_sym minu_obj_to_sym(minu_state *mrb, minu_value name) {
  return mrb_obj_to_sym(mrb, name);
}

const char *minu_sym2name(minu_state *mrb, minu_sym mid) {
  return mrb_sym2name(mrb, mid);
}
CODE