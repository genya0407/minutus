use crate::test_utils::Executor;
use minutus::mruby::*;

#[minutus::wrap(class_method = "new", method = "values")]
struct TupleWrapper1 {
    values: (String, i64),
}

impl TupleWrapper1 {
    #[minutus::class_method]
    pub fn new(values: (String, i64)) -> Self {
        Self { values: values }
    }

    #[minutus::method]
    pub fn values(&self) -> (String, i64) {
        self.values.clone()
    }
}

#[minutus::wrap(class_method = "new", method = "values")]
struct TupleWrapper2 {
    values: (
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
    ),
}

impl TupleWrapper2 {
    #[minutus::class_method]
    pub fn new(
        values: (
            String,
            i64,
            String,
            i64,
            String,
            i64,
            String,
            i64,
            String,
            i64,
            String,
            i64,
            String,
            i64,
            String,
            i64,
        ),
    ) -> Self {
        Self { values: values }
    }

    #[minutus::method]
    pub fn values(
        &self,
    ) -> (
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
    ) {
        self.values.clone()
    }
}

fn executor() -> Executor {
    Executor {
        initializer: |mrb: *mut minu_state| {
            TupleWrapper1::define_class_on_mrb(mrb);
            TupleWrapper2::define_class_on_mrb(mrb);
        },
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
            cnt = TupleWrapper1.new(['value1', 128])
            assert_eq(cnt.values, ['value1', 128])
            ",
    );

    executor().execute(
        "
            assert_raise_error(ArgumentError) do
              TupleWrapper1.new(['value1', '128'])
            end
            ",
    );

    executor().execute(
        "
            cnt = TupleWrapper2.new(['value1', 128]*8)
            assert_eq(cnt.values, ['value1', 128]*8)
            ",
    );
}
