use minutus::mruby::*;

#[test]
fn test_load_string_exception() {
    unsafe {
        let mrb = minu_open();

        // mrb.exc is null here
        assert!((*mrb).exc.is_null());

        // this does not cause panic nor abort
        minu_load_string(mrb, "raise 'error!'\0".as_ptr() as _);

        // mrb.exc is not null here
        assert!(!(*mrb).exc.is_null());
    }
}

#[test]
fn test_funcall_exception() {
    unsafe {
        let mrb = minu_open();

        // This causes exception, but does not panic nor abort
        let result = minu_funcall(
            mrb,
            minu_obj_value((*mrb).top_self as _),
            "nonexistent_method\0".as_ptr() as _,
            0,
        );
        assert!(minu_exception_p(result));
    }
}

#[allow(dead_code)]
unsafe extern "C" fn callback(mrb: *mut minu_state, _val: minu_value) -> minu_value {
    // This raises error, and will caught by protect
    minu_raise(mrb, (*mrb).eStandardError_class, "errro!\0".as_ptr() as _);
}

#[test]
fn test_protect() {
    use minutus::mruby::*;

    unsafe {
        let mrb = minu_open();

        let mut state = false;
        // callback raises exception, and minu_protect catches it
        let result = minu_protect(mrb, Some(callback), minu_nil_value(), &mut state);

        assert!(state);
        assert!(minu_exception_p(result));
    }
}

#[test]
fn test_raises_in_mrb() {
    use minutus::mruby::*;

    unsafe {
        let mrb = minu_open();

        if std::env::var("RUN_RAISE").is_ok() {
            // This raises error, and aborts process
            minu_raise(mrb, (*mrb).eStandardError_class, "errro!\0".as_ptr() as _);
        }
    }
}

/*
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (/Users/sangenya/dev/minutus/target/debug/deps/test_abort-8f671ec3e349e0da)

running 4 tests
test test_protect ... ok
test test_funcall_exception ... ok
test test_raises_in_mrb ... ok
test test_load_string_exception ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests test-abort

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/

/*
$ RUN_RAISE=1 cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (/Users/sangenya/dev/minutus/target/debug/deps/test_abort-8f671ec3e349e0da)

running 3 tests
errro! (StandardError)
error: test failed, to rerun pass '--lib'

Caused by:
  process didn't exit successfully: `/Users/sangenya/dev/minutus/target/debug/deps/test_abort-8f671ec3e349e0da` (signal: 6, SIGABRT: process abort signal)
 */
