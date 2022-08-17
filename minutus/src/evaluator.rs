use crate::mruby::*;
use crate::types::*;

/// Evaluates mruby codes.
///
/// # Example
///
/// ```
/// use minutus::types::*;
///
/// let runtime = minutus::Evaluator::build();
/// // prints [1,2,3] and returns `MrbValue` which holds `[1,2,3]`
/// let mruby_array = runtime.evaluate("p [1,2,3]").unwrap();
/// let array = <Vec<i64>>::try_from_mrb(mruby_array).unwrap();
/// assert_eq!(vec![1,2,3], array);
///
/// // evaluates script and returns the value as String
/// let evaluated_string = runtime.eval_to::<String>("'this is mruby string!'").unwrap();
/// assert_eq!("this is mruby string!", evaluated_string);
/// ```
pub struct Evaluator {
    mrb: *mut minu_state,
}

impl Drop for Evaluator {
    fn drop(&mut self) {
        unsafe { minu_close(self.mrb) }
    }
}

impl Evaluator {
    pub fn build() -> Self {
        unsafe {
            let mrb = minu_open();
            Self { mrb }
        }
    }

    pub fn mrb(&self) -> *mut minu_state {
        self.mrb
    }

    /// Evaluates `script` in mruby world, and returns the last evaluated value.
    pub fn evaluate(&self, script: &str) -> MrbResult<MrbValue> {
        self.eval_to::<MrbValue>(script)
    }

    /// Evaluates `script` in mruby world, and converts the last evaluated value into the specified type.
    pub fn eval_to<RETVAL: TryFromMrb>(&self, script: &str) -> MrbResult<RETVAL> {
        use crate::types::*;

        unsafe {
            let script_cstr = std::ffi::CString::new(script).unwrap();
            let retval = minu_load_string(self.mrb, script_cstr.as_ptr());

            if !(*self.mrb).exc.is_null() {
                let inspected_exception =
                    minu_inspect(self.mrb, minu_obj_value((*self.mrb).exc as _));
                let msg = String::try_from_mrb(MrbValue::new(self.mrb, inspected_exception))
                    .expect("Failed to convert a exception into String");
                return Err(MrbConversionError { msg });
            }

            RETVAL::try_from_mrb(MrbValue::new(self.mrb, retval))
        }
    }
}
