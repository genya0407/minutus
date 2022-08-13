use super::*;

impl<T: TryFromMrb> TryFromMrb for Option<T> {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            if minu_nil_p(value.val) {
                return Ok(None);
            } else {
                Ok(Some(T::try_from_mrb(value)?))
            }
        }
    }
}

impl<T: TryIntoMrb> TryIntoMrb for Option<T> {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe {
            if let Some(v) = self {
                v.try_into_mrb(mrb)
            } else {
                Ok(MrbValue::new(mrb, minu_nil_value()))
            }
        }
    }
}
