puts <<~CODE
struct RClass *minu_class_get(minu_state *mrb, const char *name) {
  return mrb_class_get(mrb, name);
}

struct RClass *minu_define_class(minu_state *mrb, const char *name,
                                 struct RClass *super) {
  return mrb_define_class(mrb, name, super);
}

minu_value minu_load_string(minu_state *mrb, const char *s) {
  return mrb_load_string(mrb, s);
}

minu_state *minu_open() { return mrb_open(); }

void minu_close(minu_state *mrb) { return mrb_close(mrb); }

minu_value minu_inspect(minu_state *mrb, mrb_value v) { return mrb_inspect(mrb, v); }

minu_bool minu_obj_is_kind_of(minu_state * mrb, minu_value obj, struct RClass * c) {
  return mrb_obj_is_kind_of(mrb, obj, c);
}

minu_value minu_funcall_argv(minu_state * mrb, minu_value val, minu_sym name, minu_int argc, const minu_value * argv) {
  return mrb_funcall_argv(mrb, val, name, argc, argv);
}

minu_value minu_funcall_with_block(minu_state * mrb, minu_value val, minu_sym name, minu_int argc, const minu_value * argv, mrb_value block) {
  return mrb_funcall_with_block(mrb, val, name, argc, argv, block);
}
CODE
