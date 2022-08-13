//! Type casting logics between Rust and mruby

use crate::mruby::*;

mod array;
mod bool;
mod float;
mod hash;
mod integer;
mod option;
mod r_symbol;
mod string;
mod tuples;
mod unit;
mod values;

pub use r_symbol::*;
pub use values::*;

/// Trait that handles casting Rust value into mruby value.
pub trait IntoMrb {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value;
}

/// Trait that handles casting mruby value into Rust value.
pub trait FromMrb<Target> {
    fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Target;
}
