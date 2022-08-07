use crate::test_utils::Executor;
use minutus::mruby::*;
use std::collections::HashMap;

#[minutus::wrap(class_method = "new", method = "values")]
struct HashWrapper {
    values: HashMap<String, i64>,
}

impl HashWrapper {
    #[minutus::class_method]
    pub fn new(values: HashMap<String, i64>) -> Self {
        Self { values: values }
    }

    #[minutus::method]
    pub fn values(&self) -> HashMap<String, i64> {
        self.values.clone()
    }
}

fn executor() -> Executor {
    Executor {
        initializer: |mrb: *mut minu_state| {
            HashWrapper::define_class_on_mrb(mrb);
        },
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
        cnt = HashWrapper.new('key1' => 1)
        assert_eq(cnt.values, {'key1' => 1})
        ",
    );

    executor().execute(
        "
            assert_raise_error(ArgumentError) do
            HashWrapper.new('key1' => 1.0)
            end
            ",
    );
}
