use super::*;

impl IntoMrb for () {
    fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
        unsafe { minu_nil_value() }
    }
}

impl FromMrb<()> for () {
    fn from_mrb(_mrb: *mut minu_state, _val: &minu_value) -> () {}
}
