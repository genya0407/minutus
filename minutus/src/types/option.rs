use super::*;

impl<T: FromMrb<T>> FromMrb<Self> for Option<T> {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
        unsafe {
            if minu_nil_p(*value) {
                return None;
            } else {
                Some(T::from_mrb(mrb, value))
            }
        }
    }
}

impl<T: IntoMrb> IntoMrb for Option<T> {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
        unsafe {
            if let Some(v) = self {
                v.into_mrb(mrb)
            } else {
                minu_nil_value()
            }
        }
    }
}
