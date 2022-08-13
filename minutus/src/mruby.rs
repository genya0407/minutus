//! Low-level interface to mruby C API.
//!
//! Most functions/types are almost identical to the original mruby C API. Only difference is their prefix (`minu_` instead of `mrb_`).
//!
//! see: [mruby documentation](https://mruby.org/docs/api/headers.html)

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// types
pub use internal::{minu_aspec, minu_bool, minu_float, minu_int, minu_state, minu_sym, minu_value};
pub use internal::{RArray, RBasic, RClass, RData, RException, RHash, RObject, RProc, RString};

// predicates
pub use internal::{
    minu_array_p, minu_class_p, minu_data_p, minu_exception_p, minu_false_p, minu_fixnum_p,
    minu_float_p, minu_hash_p, minu_module_p, minu_nil_p, minu_object_p, minu_range_p,
    minu_string_p, minu_true_p,
};

// generates minu_value
pub use internal::{minu_false_value, minu_nil_value, minu_true_value};
// convert to minu_value
pub use internal::{minu_fixnum_value, minu_float_value, minu_obj_value};
// convert from minu_value
pub use internal::{minu_fixnum_func, minu_float_func};

// RData
pub use internal::{
    minu_MRB_ARGS_ARG, minu_data_get_ptr, minu_data_object_alloc, minu_data_type,
    minu_define_class_method, minu_define_method, minu_free, minu_malloc, minu_set_vtype_as_data,
};

// class
pub use internal::{minu_class_get, minu_define_class};

// string
pub use internal::{minu_str_new, minu_str_new_cstr, minu_str_to_cstr};

// array
pub use internal::{minu_ary_new, minu_ary_new_capa, minu_ary_push, minu_ary_ref, minu_rarray_len};

// hash
pub use internal::{
    minu_hash_keys, minu_hash_new_capa, minu_hash_set, minu_hash_size, minu_hash_values,
};

// GC
pub use internal::{minu_gc_register, minu_gc_unregister};

// Exception
pub use internal::{
    minu_exc_backtrace, minu_get_backtrace, minu_print_backtrace, minu_print_error,
};

// Symbol
pub use internal::{minu_intern_cstr, minu_obj_to_sym, minu_sym2name, minu_symbol_value};

// other
pub use internal::{minu_close, minu_inspect, minu_load_string, minu_open};

extern "C" {
    #[link_name = "mrb_get_args"]
    pub fn minu_get_args(
        mrb: *mut minu_state,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> minu_int;
}

extern "C" {
    #[link_name = "mrb_funcall"]
    pub fn minu_funcall(
        mrb: *mut minu_state,
        val: minu_value,
        name: *const ::std::os::raw::c_char,
        argc: minu_int,
        ...
    ) -> minu_value;
}

pub unsafe fn minu_raise(
    mrb: *mut minu_state,
    c: *mut RClass,
    msg: *const ::std::os::raw::c_char,
) -> ! {
    internal::mrb_raise(mrb, c, msg);
    panic!("should never come here!")
}

mod internal {
    include!(concat!(env!("OUT_DIR"), "/mruby.rs"));
}
