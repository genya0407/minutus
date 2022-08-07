puts <<~CODE
void minu_gc_register(mrb_state *mrb, mrb_value obj) {
  return mrb_gc_register(mrb, obj);
}
void minu_gc_unregister(mrb_state *mrb, mrb_value obj) {
  return mrb_gc_unregister(mrb, obj);
}
CODE