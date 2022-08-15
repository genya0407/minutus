puts <<~CODE
void minu_print_backtrace(mrb_state *mrb) {
  return mrb_print_backtrace(mrb);
}

void minu_print_error(mrb_state *mrb) {
  return mrb_print_error(mrb);
}

minu_value minu_get_backtrace(mrb_state *mrb) {
  return mrb_get_backtrace(mrb);
}

minu_value minu_exc_backtrace(mrb_state *mrb, mrb_value exc) {
  return mrb_exc_backtrace(mrb, exc);
}

minu_value minu_protect(minu_state * mrb, minu_func_t body, minu_value data, minu_bool * state) {
  return mrb_protect(mrb, body, data, state);
}
CODE