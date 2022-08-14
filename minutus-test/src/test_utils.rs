use minutus::mruby::*;

pub struct Executor {
    pub initializer: fn(*mut minu_state),
}

impl Executor {
    pub fn execute(&self, script: &str) {
        let evaluator = minutus::Evaluator::build();
        (self.initializer)(evaluator.mrb());

        evaluator
            .evaluate(
                "
            def assert_eq(a, b); raise \"#{a} is not equal to #{b}\" unless a == b; end
            def assert_raise_error(error_class)
              error = nil
              begin
                yield
              rescue error_class => e
                error = e
              end

              raise \"Error #{error_class} did not raised.\" unless error
            end
            ",
            )
            .unwrap();
        match evaluator.evaluate(script) {
            Ok(_) => (),
            Err(msg) => panic!("test failed: {}", msg),
        }
    }
}
