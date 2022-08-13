//! Type casting logics between Rust and mruby
//!
//! | Rust type | mruby type |
//! |-----------|------------|
//! | `i8`, `i16`, `i32`, `i64`, `isize` | `Integer` |
//! | `u8`, `u16`, `u32`, `u64`, `usize` | `Integer` |
//! | `f32`, `f64` | `Float` |
//! | `String` | `String` |
//! | `Option<T>` | `T` or `nil` |
//! | `(T, U)`, `(T, U, V)`, etc | `[T, U]`, `[T, U, V]`, etc |
//! | `Vec<T>` | `Array` |
//! | `std::collections::HashMap<T, U>` | `Hash` |
//! | `minutus::types::RSymbol` | `Symbol` |
//! | `bool` | any object |
//! | `MrbData` (structs marked by `minutus::wrap`) | corresponding class |
//!
//! Any value in mruby can be cast to Rust's `bool`.
//! Rust's `bool` cast to mruby's `true` or `false`.

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
