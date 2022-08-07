use crate::test_utils::Executor;
use minutus::mruby::*;

#[minutus::wrap(class_method = "new", method = "push_string", method = "values")]
struct ArrayString {
    values: Vec<String>,
}

impl ArrayString {
    #[minutus::class_method]
    pub fn new(values: Vec<String>) -> Self {
        Self { values: values }
    }

    #[minutus::method]
    pub fn push_string(&self, value: String) -> Self {
        let mut new_values = self.values.clone();
        new_values.push(value);
        Self { values: new_values }
    }

    #[minutus::method]
    pub fn values(&self) -> Vec<String> {
        self.values.clone()
    }
}

fn executor() -> Executor {
    Executor {
        initializer: |mrb: *mut minu_state| {
            ArrayString::define_class_on_mrb(mrb);
        },
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
            cnt = ArrayString.new(['value1'])
            cnt = cnt.push_string('value2')
            cnt = cnt.push_string('value3')
            assert_eq(cnt.values, %w[value1 value2 value3])
            ",
    );

    executor().execute(
        "
            cnt = ArrayString.new([])
            cnt.push_string('value1')
            assert_raise_error(ArgumentError) do
                cnt.push_string(200)
            end
            ",
    );
}
