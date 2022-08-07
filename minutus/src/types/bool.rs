use super::*;

impl FromMrb<Self> for bool {
    fn from_mrb(_mrb: *mut minu_state, value: &minu_value) -> Self {
        unsafe {
            if minu_false_p(*value) || minu_nil_p(*value) {
                false
            } else {
                true
            }
        }
    }
}

impl IntoMrb for bool {
    fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
        unsafe {
            if self {
                minu_true_value()
            } else {
                minu_false_value()
            }
        }
    }
}
