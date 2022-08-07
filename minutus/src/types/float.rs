use super::*;

macro_rules! define_float {
    ($t:ty) => {
        impl FromMrb<Self> for $t {
            fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> $t {
                unsafe {
                    if minu_float_p(*value) {
                        minu_float_func(*value) as $t
                    } else {
                        crate::utils::raise_type_mismatch_argument_error(mrb, *value, "Float")
                    }
                }
            }
        }

        impl IntoMrb for $t {
            fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
                unsafe { minu_float_value(mrb, self as f64) }
            }
        }
    };
}

define_float!(f64);
define_float!(f32);
