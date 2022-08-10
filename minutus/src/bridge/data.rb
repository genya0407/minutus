puts <<~CODE
typedef mrb_data_type minu_data_type;

void *minu_data_get_ptr(minu_state *mrb, minu_value v,
                        const minu_data_type *t)
{
  return mrb_data_get_ptr(mrb, v, t);
}

void *minu_malloc(mrb_state *mrb, size_t s) { return mrb_malloc(mrb, s); }

struct RData *minu_data_object_alloc(mrb_state *mrb, struct RClass *klass,
                                     void *datap, const mrb_data_type *type)
{
  return mrb_data_object_alloc(mrb, klass, datap, type);
}

void minu_free(minu_state *mrb, void *ptr) { mrb_free(mrb, ptr); }

void minu_set_vtype_as_data(struct RClass *cla)
{
  MRB_SET_INSTANCE_TT(cla, MRB_TT_DATA);
}
CODE