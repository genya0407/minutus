puts <<~CODE
void minu_define_class_method(minu_state *mrb, struct RClass *cla,
  const char *name, mrb_func_t fun,
  minu_aspec aspec) {
mrb_define_class_method(mrb, cla, name, fun, aspec);
}

void minu_define_method(minu_state *mrb, struct RClass *cla, const char *name,
mrb_func_t func, minu_aspec aspec) {
mrb_define_method(mrb, cla, name, func, aspec);
}

minu_aspec minu_MRB_ARGS_ARG(uint32_t n1, uint32_t n2) {
return MRB_ARGS_ARG(n1, n2);
}
CODE