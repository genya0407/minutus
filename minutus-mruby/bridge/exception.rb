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
CODE