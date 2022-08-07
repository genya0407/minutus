use crate::mruby::*;

macro_rules! cstr {
    ($lit:literal) => {
        std::ffi::CString::new($lit).unwrap().as_ptr()
    };
}
pub(crate) use cstr;

pub unsafe fn raise_type_mismatch_argument_error(
    mrb: *mut minu_state,
    value: minu_value,
    type_name: &str,
) -> ! {
    let value_inspected = minu_inspect(mrb, value);
    let value_inspected_str =
        std::ffi::CStr::from_ptr(crate::mruby::minu_str_to_cstr(mrb, value_inspected));
    let message = format!(
        "{} cannot be converted into {}",
        value_inspected_str.to_str().unwrap(),
        type_name
    );
    let message_cstr = std::ffi::CString::new(message).unwrap();
    crate::mruby::minu_raise(
        mrb,
        crate::mruby::minu_class_get(mrb, crate::utils::cstr!("ArgumentError")),
        message_cstr.as_ptr(),
    )
}
