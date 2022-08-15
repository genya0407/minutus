use super::*;

impl TryFromMrb for String {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            if minu_string_p(value.val) {
                let cstr = std::ffi::CStr::from_ptr(minu_str_to_cstr(value.mrb, value.val));
                Ok(cstr.to_string_lossy().into_owned())
            } else {
                let inspected = minu_inspect(value.mrb, value.val);
                let inspected_string = String::try_from_mrb(MrbValue::new(value.mrb, inspected))?;
                Err(MrbConversionError {
                    msg: format!("{} could not converted into String", inspected_string),
                })
            }
        }
    }
}

impl TryIntoMrb for String {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        let cstr = std::ffi::CString::new(self).unwrap();
        unsafe { Ok(MrbValue::new(mrb, minu_str_new_cstr(mrb, cstr.as_ptr()))) }
    }
}

impl TryIntoMrb for &str {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        let cstr = std::ffi::CString::new(self).unwrap();
        unsafe { Ok(MrbValue::new(mrb, minu_str_new_cstr(mrb, cstr.as_ptr()))) }
    }
}

// impl TryFromMrb for Vec<u8> {
//     fn try_from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
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

// impl TryIntoMrb for Vec<u8> {
//     fn try_into_mrb(self, mrb: *mut minu_state) -> minu_value {
//         unsafe {
//             minu_str_new(
//                 mrb,
//                 self.as_ptr() as *const _,
//                 self.len().try_into().unwrap(),
//             )
//         }
//     }
// }
