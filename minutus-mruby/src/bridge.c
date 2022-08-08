#include "mruby.h"
#include "mruby/array.h"
#include "mruby/class.h"
#include "mruby/compile.h"
#include "mruby/data.h"
#include "mruby/error.h"
#include "mruby/hash.h"
#include "mruby/string.h"
#include "mruby/value.h"

// In order to use mrb_get_backtrace in mruby master
// (a0c02e0a6465ff9f37b7b2e4801081cef7c0e93c).
#if __has_include("mruby/internal.h")
#include "mruby/internal.h"
#endif

typedef mrb_state minu_state;

typedef mrb_aspec minu_aspec;

typedef mrb_value minu_value;

typedef mrb_int minu_int;

typedef mrb_float minu_float;

typedef mrb_bool minu_bool;

minu_bool minu_array_p(minu_value v) { return mrb_array_p(v); }

minu_bool minu_class_p(minu_value v) { return mrb_class_p(v); }

minu_bool minu_data_p(minu_value v) { return mrb_data_p(v); }

minu_bool minu_exception_p(minu_value v) { return mrb_exception_p(v); }

minu_bool minu_false_p(minu_value v) { return mrb_false_p(v); }

minu_bool minu_fixnum_p(minu_value v) { return mrb_fixnum_p(v); }

minu_bool minu_float_p(minu_value v) { return mrb_float_p(v); }

minu_bool minu_hash_p(minu_value v) { return mrb_hash_p(v); }

minu_bool minu_module_p(minu_value v) { return mrb_module_p(v); }

minu_bool minu_nil_p(minu_value v) { return mrb_nil_p(v); }

minu_bool minu_object_p(minu_value v) { return mrb_object_p(v); }

minu_bool minu_range_p(minu_value v) { return mrb_range_p(v); }

minu_bool minu_string_p(minu_value v) { return mrb_string_p(v); }

minu_bool minu_true_p(minu_value v) { return mrb_true_p(v); }

minu_value minu_false_value() { return mrb_false_value(); }

minu_value minu_nil_value() { return mrb_nil_value(); }

minu_value minu_true_value() { return mrb_true_value(); }

// _func postfix because of symbol duplication
minu_float minu_float_func(minu_value v) { return mrb_float(v); }

// _func postfix because of symbol duplication
minu_int minu_fixnum_func(minu_value v) { return mrb_fixnum(v); }

minu_value minu_fixnum_value(minu_int v) { return mrb_fixnum_value(v); }

minu_value minu_float_value(minu_state *minu, minu_float v) {
  return mrb_float_value(minu, v);
}

minu_value minu_obj_value(void *v) { return mrb_obj_value(v); }
typedef mrb_data_type minu_data_type;

void *minu_data_get_ptr(minu_state *mrb, minu_value v,
                        const minu_data_type *t) {
  return mrb_data_get_ptr(mrb, v, t);
}

void *minu_malloc(mrb_state *mrb, size_t s) { return mrb_malloc(mrb, s); }

struct RData *minu_data_object_alloc(mrb_state *mrb, struct RClass *klass,
                                     void *datap, const mrb_data_type *type) {
  return mrb_data_object_alloc(mrb, klass, datap, type);
}

void minu_free(minu_state *mrb, void *ptr) { mrb_free(mrb, ptr); }

void minu_set_vtype_as_data(struct RClass *cla) {
  MRB_SET_INSTANCE_TT(cla, MRB_TT_DATA);
}
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
char *minu_str_to_cstr(mrb_state *mrb, mrb_value str) {
  return mrb_str_to_cstr(mrb, str);
}
mrb_value minu_str_new_cstr(mrb_state *mrb, const char *s) {
  return mrb_str_new_cstr(mrb, s);
}
mrb_value minu_str_new(mrb_state *mrb, const char *s, size_t len) {
  return mrb_str_new(mrb, s, len);
}
minu_int minu_rarray_len(minu_value ary) { return RARRAY_LEN(ary); }

minu_value minu_ary_ref(minu_value ary, minu_int n) {
  return mrb_ary_entry(ary, n);
}

minu_value minu_ary_new_capa(mrb_state *mrb, mrb_int capa) {
  return mrb_ary_new_capa(mrb, capa);
}

minu_value minu_ary_new(mrb_state *mrb) { return mrb_ary_new(mrb); }

void minu_ary_push(mrb_state *mrb, mrb_value array, mrb_value value) {
  return mrb_ary_push(mrb, array, value);
}
minu_value minu_hash_keys(minu_state *mrb, minu_value hash) {
  return mrb_hash_keys(mrb, hash);
}
minu_value minu_hash_values(minu_state *mrb, minu_value hash) {
  return mrb_hash_values(mrb, hash);
}
minu_int minu_hash_size(minu_state *mrb, minu_value hash) {
  return mrb_hash_size(mrb, hash);
}
minu_value minu_hash_new_capa(minu_state *mrb, minu_int capa) {
  return mrb_hash_new_capa(mrb, capa);
}
void minu_hash_set(minu_state *mrb, minu_value hash, minu_value key,
                   minu_value val) {
  return mrb_hash_set(mrb, hash, key, val);
}
void minu_gc_register(mrb_state *mrb, mrb_value obj) {
  return mrb_gc_register(mrb, obj);
}
void minu_gc_unregister(mrb_state *mrb, mrb_value obj) {
  return mrb_gc_unregister(mrb, obj);
}
void minu_print_backtrace(mrb_state *mrb) { return mrb_print_backtrace(mrb); }

void minu_print_error(mrb_state *mrb) { return mrb_print_error(mrb); }

minu_value minu_get_backtrace(mrb_state *mrb) { return mrb_get_backtrace(mrb); }

minu_value minu_exc_backtrace(mrb_state *mrb, mrb_value exc) {
  return mrb_exc_backtrace(mrb, exc);
}
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

minu_value minu_inspect(minu_state *mrb, mrb_value v) {
  return mrb_inspect(mrb, v);
}
