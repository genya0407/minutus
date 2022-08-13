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

pub use r_symbol::*;

pub type MrbResult<T> = Result<T, MrbConversionError>;

/// Trait that handles casting Rust value into mruby value.
pub trait TryIntoMrb {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue>;
}

/// Trait that handles casting mruby value into Rust value.
pub trait TryFromMrb<Target = Self>: Sized {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Target>;
}

/// Represents values returned from mruby world.
///
/// Using `minutus::define_funcall` macro, you can define arbitrary methods to this type.
#[derive(Clone, Debug)]
pub struct MrbValue {
    pub mrb: *mut minu_state,
    pub val: minu_value,
}

impl MrbValue {
    pub fn new(mrb: *mut minu_state, val: minu_value) -> Self {
        Self { mrb, val }
    }
}

impl TryIntoMrb for MrbValue {
    fn try_into_mrb(self, _mrb: *mut minu_state) -> MrbResult<MrbValue> {
        Ok(self)
    }
}

impl TryFromMrb for MrbValue {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        Ok(value)
    }
}

impl std::fmt::Display for MrbConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for MrbConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct MrbConversionError {
    pub msg: String,
}

impl MrbConversionError {
    pub fn new(ty: &str) -> Self {
        Self {
            msg: format!("Could not convert into {}", ty),
        }
    }
}
