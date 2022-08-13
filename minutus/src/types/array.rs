use super::*;

impl<T: TryFromMrb> TryFromMrb for Vec<T> {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            if minu_array_p(value.val) {
                let len = minu_rarray_len(value.val);
                let mut vec = Vec::with_capacity(len as _);
                for i in 0..len {
                    let minu_val = minu_ary_ref(value.val, i);
                    let val = T::try_from_mrb(MrbValue::new(value.mrb, minu_val))?;
                    vec.push(val);
                }
                Ok(vec)
            } else {
                Err(MrbConversionError::new("Vec<T>"))
            }
        }
    }
}

impl<T: TryIntoMrb> TryIntoMrb for Vec<T> {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe {
            let ary = minu_ary_new_capa(mrb, self.len() as _);
            for v in self.into_iter() {
                minu_ary_push(mrb, ary, v.try_into_mrb(mrb)?.val)
            }
            return Ok(MrbValue::new(mrb, ary));
        }
    }
}
