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

/// Represents values returned from mruby world.
///
/// Using `minutus::define_funcall` macro, you can define arbitrary methods to this type.
#[derive(Clone, Debug)]
pub struct MinuValue {
    pub mrb: *mut minu_state,
    pub val: minu_value,
}

impl IntoMrb for MinuValue {
    fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
        self.val
    }
}

impl FromMrb<MinuValue> for MinuValue {
    fn from_mrb(mrb: *mut minu_state, val: &minu_value) -> Self {
        Self { mrb, val: *val }
    }
}
