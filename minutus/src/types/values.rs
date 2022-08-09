use super::*;

impl IntoMrb for minu_value {
    fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
        self
    }
}

impl FromMrb<minu_value> for minu_value {
    fn from_mrb(_mrb: *mut minu_state, val: &minu_value) -> minu_value {
        *val
    }
}
