use super::*;

macro_rules! define_float {
    ($t:ty) => {
        impl TryFromMrb for $t {
            fn try_from_mrb(value: MrbValue) -> MrbResult<$t> {
                unsafe {
                    if minu_float_p(value.val) {
                        Ok(minu_float_func(value.val) as $t)
                    } else {
                        Err(MrbConversionError::new("Float"))
                    }
                }
            }
        }

        impl TryIntoMrb for $t {
            fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
                unsafe { Ok(MrbValue::new(mrb, minu_float_value(mrb, self as f64))) }
            }
        }
    };
}

define_float!(f64);
define_float!(f32);
