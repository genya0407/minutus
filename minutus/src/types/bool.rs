use super::*;

impl TryFromMrb for bool {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            if minu_false_p(value.val) || minu_nil_p(value.val) {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }
}

impl TryIntoMrb for bool {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe {
            if self {
                Ok(MrbValue::new(mrb, minu_true_value()))
            } else {
                Ok(MrbValue::new(mrb, minu_false_value()))
            }
        }
    }
}
