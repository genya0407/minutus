use super::*;

macro_rules! define_int {
    ($t:ty) => {
        impl FromMrb<Self> for $t {
            fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> $t {
                unsafe {
                    if minu_fixnum_p(*value) {
                        minu_fixnum_func(*value) as $t
                    } else {
                        crate::utils::raise_type_mismatch_argument_error(mrb, *value, "Fixnum")
                    }
                }
            }
        }

        impl IntoMrb for $t {
            fn into_mrb(self, _mrb: *mut minu_state) -> minu_value {
                unsafe { minu_fixnum_value(self as i64) }
            }
        }
    };
}

define_int!(i64);
define_int!(i32);
define_int!(i16);
define_int!(i8);
define_int!(isize);

define_int!(u64);
define_int!(u32);
define_int!(u16);
define_int!(u8);
define_int!(usize);
