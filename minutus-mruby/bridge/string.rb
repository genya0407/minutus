puts <<~CODE
char * minu_str_to_cstr(mrb_state * mrb, mrb_value str) {
  return mrb_str_to_cstr(mrb, str);
}
mrb_value minu_str_new_cstr(mrb_state * mrb, const char *s) {
  return mrb_str_new_cstr(mrb, s);
}
mrb_value minu_str_new(mrb_state * mrb, const char *s, size_t len) {
  return mrb_str_new(mrb, s, len);
}
CODE