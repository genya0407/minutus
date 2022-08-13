use super::*;

pub trait ToSymbol {
    fn to_sym(&self, mrb: *mut minu_state) -> RSymbol;
}

impl ToSymbol for &str {
    fn to_sym(&self, mrb: *mut minu_state) -> RSymbol {
        RSymbol::new(mrb, self)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RSymbol {
    mid: minu_sym,
}

impl RSymbol {
    pub fn new(mrb: *mut minu_state, s: &str) -> Self {
        unsafe {
            let cstr = std::ffi::CString::new(s).unwrap();
            let mid = minu_intern_cstr(mrb, cstr.as_ptr());
            Self { mid }
        }
    }

    pub fn to_string(&self, mrb: *mut minu_state) -> String {
        unsafe {
            let p = minu_sym2name(mrb, self.mid);
            let cstr = std::ffi::CStr::from_ptr(p);
            cstr.to_string_lossy().into_owned()
        }
    }
}

impl FromMrb<RSymbol> for RSymbol {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
        unsafe {
            let mid = minu_obj_to_sym(mrb, *value);
            Self { mid }
        }
    }
}

impl IntoMrb for RSymbol {
    fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
        unsafe { minu_symbol_value(self.mid) }
    }
}
