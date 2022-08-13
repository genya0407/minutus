use super::*;

impl TryIntoMrb for () {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe { Ok(MrbValue::new(mrb, minu_nil_value())) }
    }
}

impl TryFromMrb for () {
    fn try_from_mrb(_value: MrbValue) -> MrbResult<()> {
        Ok(())
    }
}
