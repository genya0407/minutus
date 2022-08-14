use super::*;

/// Extension trait that defines `to_sym` method.
pub trait ToSymbol {
    fn to_sym(&self, mrb: *mut minu_state) -> RSymbol;
}

impl ToSymbol for &str {
    fn to_sym(&self, mrb: *mut minu_state) -> RSymbol {
        RSymbol::new(mrb, self)
    }
}

/// Represents mruby's symbol.
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

impl TryFromMrb for RSymbol {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            // TODO: type check
            let mid = minu_obj_to_sym(value.mrb, value.val);
            Ok(Self { mid })
        }
    }
}

impl TryIntoMrb for RSymbol {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe { Ok(MrbValue::new(mrb, minu_symbol_value(self.mid))) }
    }
}
