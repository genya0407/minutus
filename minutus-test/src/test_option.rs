use crate::test_utils::Executor;
use minutus::mruby::*;

#[minutus::wrap(class_method = "new", method = "value")]
struct OptionalString {
    val: Option<String>,
}

impl OptionalString {
    #[minutus::class_method]
    pub fn new(val: Option<String>) -> Self {
        Self { val: val }
    }

    #[minutus::method]
    pub fn value(&self) -> Option<String> {
        self.val.clone()
    }
}

fn executor() -> Executor {
    Executor {
        initializer: |mrb: *mut minu_state| {
            OptionalString::define_class_on_mrb(mrb);
        },
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
            opt = OptionalString.new('aaa')
            assert_eq(opt.value, 'aaa')
            ",
    );

    executor().execute(
        "
            opt = OptionalString.new(nil)
            assert_eq(opt.value, nil)
            ",
    );

    executor().execute(
        "
            assert_raise_error(ArgumentError) do
                OptionalString.new(100)
            end
            ",
    );
}
