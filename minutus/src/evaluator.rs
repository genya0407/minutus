use crate::mruby::*;
use crate::types::*;

/// Builds [`Evaluator`].
///
/// If you need more customizable [`Evaluator`], see [`Evaluator::build()`]
pub fn build_simple_evaluator() -> Evaluator<MinuValue> {
    Evaluator::<MinuValue>::build(|_| {}, MinuValue::from_mrb)
}

/// Evaluates mruby code, and holds `mrb`.
///
/// You can build fully customizable `Evaluator` by `Evaluator::build`.
/// Or you can build more simplified one by `build_simple_evaluator`.
///
/// # Example
///
/// `build_simple_evaluator` example:
///
/// ```
/// minutus::define_funcall!{
///   fn concat(&self, other: Vec<i64>) -> Vec<i64> => "+";
/// }
///
/// fn main() {
///     let runtime = minutus::build_simple_evaluator();
///     // prints [1,2,3] and returns `MinuValue` which holds `[1,2,3]`
///     let array = runtime.evaluate("p [1,2,3]").unwrap();
///     // `concat` returns Vec<i64> because of the `define_funcall` definition.
///     let concat_array = array.concat(vec![4,5,6]);
///     assert_eq!(vec![1,2,3,4,5,6], concat_array);
/// }
/// ```
///
/// `build` example:
///
/// ```
/// fn main() {
///     use minutus::types::*; // for using bool::from_mrb
///
///     // `init` is executed when `Evaluator` is initialized.
///     let init = |mrb| {
///         let script = "def random_value; rand(100); end";
///         let cstr = std::ffi::CString::new(script).unwrap();
///         unsafe { minutus::mruby::minu_load_string(mrb, cstr.as_ptr()) };
///     };
///     // `from_mrb` is used to type-cast result of `evaluate`.
///     let from_mrb = bool::from_mrb;
///     let runtime = minutus::Evaluator::build(init, from_mrb);
///
///     assert_eq!(true, runtime.evaluate("true").unwrap());
///     assert_eq!(false, runtime.evaluate("false").unwrap());
///     assert_eq!(false, runtime.evaluate("nil").unwrap());
///     assert_eq!(true, runtime.evaluate("0").unwrap())
/// }
/// ```
pub struct Evaluator<EvaluationResult> {
    mrb: *mut minu_state,
    from_mrb: fn(*mut minu_state, &minu_value) -> EvaluationResult,
}

impl<EvaluationResult> Drop for Evaluator<EvaluationResult> {
    fn drop(&mut self) {
        unsafe { minu_close(self.mrb) }
    }
}

impl<EvaluationResult> Evaluator<EvaluationResult> {
    pub fn build(
        initializer: fn(*mut minu_state),
        from_mrb: fn(*mut minu_state, &minu_value) -> EvaluationResult,
    ) -> Self {
        unsafe {
            let mrb = minu_open();
            initializer(mrb);
            Self { mrb, from_mrb }
        }
    }

    pub fn mrb(&self) -> *mut minu_state {
        self.mrb
    }

    /// Evaluates `script` in mruby world, and type-cast its return value.
    ///
    /// - `EvaluationResult` is determined by `from_mrb` function passed to `build`.
    ///   - If you use `build_simple_evaluator`, `EvaluationResult` is `MinuValue`.
    /// - When an error is raised in `script`, it returns `Err(msg)`.
    pub fn evaluate(&self, script: &str) -> Result<EvaluationResult, String> {
        use crate::types::*;

        unsafe {
            let script_cstr = std::ffi::CString::new(script).unwrap();
            let retval = minu_load_string(self.mrb, script_cstr.as_ptr());

            if !(*self.mrb).exc.is_null() {
                let inspected_exception =
                    minu_inspect(self.mrb, minu_obj_value((*self.mrb).exc as _));
                type OptStr = Option<String>;
                let message =
                    OptStr::from_mrb(self.mrb, &inspected_exception).unwrap_or(String::from(""));
                return Err(message);
            }

            Ok((self.from_mrb)(self.mrb, &retval))
        }
    }
}
