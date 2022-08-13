use super::*;

macro_rules! define_int {
    ($t:ty) => {
        impl TryFromMrb for $t {
            fn try_from_mrb(value: MrbValue) -> MrbResult<$t> {
                unsafe {
                    if minu_fixnum_p(value.val) {
                        Ok(minu_fixnum_func(value.val) as $t)
                    } else {
                        Err(MrbConversionError::new("Fixnum"))
                    }
                }
            }
        }

        impl TryIntoMrb for $t {
            fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
                unsafe { Ok(MrbValue::new(mrb, minu_fixnum_value(self as i64))) }
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
