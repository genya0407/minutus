puts <<~CODE
minu_int minu_rarray_len(minu_value ary) {
  return RARRAY_LEN(ary);
}

minu_value minu_ary_ref(minu_value ary, minu_int n) {
  return mrb_ary_entry(ary, n);
}

minu_value minu_ary_new_capa(mrb_state* mrb, mrb_int capa) {
  return mrb_ary_new_capa(mrb, capa);
}

minu_value minu_ary_new(mrb_state* mrb) {
  return mrb_ary_new(mrb);
}

void minu_ary_push(mrb_state * mrb, mrb_value array, mrb_value value) {
  return mrb_ary_push(mrb, array, value);
}
CODE