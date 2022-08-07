use super::*;

impl FromMrb<Self> for String {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
        unsafe {
            if minu_string_p(*value) {
                let cstr = std::ffi::CStr::from_ptr(minu_str_to_cstr(mrb, *value));
                return cstr.to_string_lossy().into_owned();
            } else {
                crate::utils::raise_type_mismatch_argument_error(mrb, *value, "String")
            }
        }
    }
}

impl IntoMrb for String {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
        let cstr = std::ffi::CString::new(self).unwrap();
        unsafe { minu_str_new_cstr(mrb, cstr.as_ptr()) }
    }
}

// impl FromMrb<Self> for Vec<u8> {
//     fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
//         unsafe {
//             if minu_string_p(*value) {
//                 let cstr = std::ffi::CStr::from_ptr(minu_str_to_cstr(mrb, *value));
//                 return cstr.to_bytes().to_vec();
//             } else {
//                 crate::utils::raise_type_mismatch_argument_error(mrb, *value, "Vec<u8>")
//             }
//         }
//     }
// }

// impl IntoMrb for Vec<u8> {
//     fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
//         unsafe {
//             minu_str_new(
//                 mrb,
//                 self.as_ptr() as *const _,
//                 self.len().try_into().unwrap(),
//             )
//         }
//     }
// }
