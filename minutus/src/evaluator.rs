use crate::mruby::*;

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
    pub fn new(from_mrb: fn(*mut minu_state, &minu_value) -> EvaluationResult) -> Self {
        unsafe {
            Self {
                mrb: minu_open(),
                from_mrb,
            }
        }
    }

    pub fn mrb(&self) -> *mut minu_state {
        self.mrb
    }

    pub fn new_with_intializer(
        initializer: fn(*mut minu_state),
        from_mrb: fn(*mut minu_state, &minu_value) -> EvaluationResult,
    ) -> Self {
        unsafe {
            let mrb = minu_open();
            initializer(mrb);
            Self { mrb, from_mrb }
        }
    }

    pub fn evaluate(&self, script: &str) -> Result<EvaluationResult, String> {
        use crate::types::*;

        unsafe {
            let script_cstr = std::ffi::CString::new(script).unwrap();
            let retval = minu_load_string(self.mrb, script_cstr.as_ptr());

            if !(*self.mrb).exc.is_null() {
                let msg = minu_get_backtrace(self.mrb);
                type OptStr = Option<String>;
                let message = OptStr::from_mrb(self.mrb, &msg).unwrap_or(String::from(""));
                return Err(message);
            }

            Ok((self.from_mrb)(self.mrb, &retval))
        }
    }
}
