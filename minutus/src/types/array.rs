use super::*;

impl<T: FromMrb<T>> FromMrb<Self> for Vec<T> {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
        unsafe {
            if minu_array_p(*value) {
                let len = minu_rarray_len(*value);
                let mut vec = Vec::with_capacity(len as _);
                for i in 0..len {
                    let minu_val = minu_ary_ref(*value, i);
                    let val = T::from_mrb(mrb, &minu_val);
                    vec.push(val);
                }
                return vec;
            } else {
                crate::utils::raise_type_mismatch_argument_error(mrb, *value, "Vec<T>")
            }
        }
    }
}

impl<T: IntoMrb> IntoMrb for Vec<T> {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
        unsafe {
            let ary = minu_ary_new_capa(mrb, self.len() as _);
            for v in self.into_iter() {
                minu_ary_push(mrb, ary, v.into_mrb(mrb))
            }
            return ary;
        }
    }
}
