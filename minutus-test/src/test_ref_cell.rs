use crate::test_utils::Executor;
use minutus::mruby::*;
use std::cell::RefCell;

#[minutus::wrap(class_method = "new", method = "value", method = "update")]
struct MutOptString(RefCell<Option<String>>);

impl MutOptString {
    #[minutus::class_method]
    pub fn new(val: Option<String>) -> Self {
        Self(RefCell::new(val))
    }

    #[minutus::method]
    pub fn value(&self) -> Option<String> {
        self.0.borrow().clone()
    }

    #[minutus::method]
    pub fn update(&self, new_value: Option<String>) {
        self.0.replace(new_value);
    }
}

fn executor() -> Executor {
    Executor {
        initializer: |mrb: *mut minu_state| {
            MutOptString::define_class_on_mrb(mrb);
        },
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
        opt = MutOptString.new('aaa')
        assert_eq(opt.value, 'aaa')
        ",
    );

    executor().execute(
        "
        opt = MutOptString.new('aaa')
        assert_eq(opt.value, 'aaa')
        opt.update(nil)
        assert_eq(opt.value, nil)
        opt.update('bbb')
        assert_eq(opt.value, 'bbb')
        ",
    );
}
