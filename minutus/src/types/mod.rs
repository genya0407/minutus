use crate::mruby::*;

mod array;
mod bool;
mod float;
mod hash;
mod integer;
mod option;
mod string;
mod tuples;
mod unit;

pub trait IntoMrb {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value;
}

pub trait FromMrb<Target> {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Target;
}
