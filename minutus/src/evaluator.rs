use crate::mruby::*;
use crate::types::*;

/// Holds mrb_state and evaluates mruby codes.
///
/// # Example
///
/// ```
/// minutus::define_funcall!{
///   fn concat(&self, other: Vec<i64>) -> Vec<i64> => "+";
/// }
///
/// fn main() {
///     let runtime = minutus::Evaluator::build();
///     // prints [1,2,3] and returns `MrbValue` which holds `[1,2,3]`
///     let array = runtime.evaluate("p [1,2,3]").unwrap();
///     // `concat` returns Vec<i64> because of the `define_funcall` definition.
///     let concat_array = array.concat(vec![4,5,6]).unwrap();
///     assert_eq!(vec![1,2,3,4,5,6], concat_array);
/// }
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

    /// Evaluates `script` in mruby world, and returns the last value.
    pub fn evaluate(&self, script: &str) -> Result<MrbValue, String> {
        use crate::types::*;

        unsafe {
            let script_cstr = std::ffi::CString::new(script).unwrap();
            let retval = minu_load_string(self.mrb, script_cstr.as_ptr());

            if !(*self.mrb).exc.is_null() {
                let inspected_exception =
                    minu_inspect(self.mrb, minu_obj_value((*self.mrb).exc as _));
                let msg = String::try_from_mrb(MrbValue::new(self.mrb, inspected_exception))
                    .expect(
                        "Failed to convert the inspection result on the raised exception to String",
                    );
                return Err(msg);
            }

            Ok(MrbValue::new(self.mrb, retval))
        }
    }
}
