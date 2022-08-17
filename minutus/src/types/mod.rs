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

pub trait IntoArgs {
    type Output: AsRef<[minu_value]>;

    fn into_args(self, mrb: *mut minu_state) -> MrbResult<Self::Output>;
}

impl IntoArgs for () {
    type Output = [minu_value; 0];

    fn into_args(self, _mrb: *mut minu_state) -> MrbResult<Self::Output> {
        Ok([])
    }
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

    /// Call mruby's method.
    ///
    /// ```
    /// let runtime = minutus::Evaluator::build();
    /// let int_123 = runtime.evaluate("123").unwrap();
    ///
    /// let inspected: String = int_123.call("inspect", ()).unwrap();
    /// assert_eq!("123", inspected);
    ///
    /// let plus_result: i64 = int_123.call("+", (100,)).unwrap();
    /// assert_eq!(223, plus_result);
    /// ```
    pub fn call<ARGS: IntoArgs, RETVAL: TryFromMrb>(
        &self,
        name: &str,
        args: ARGS,
    ) -> MrbResult<RETVAL> {
        let args = args.into_args(self.mrb)?;
        let argv = args.as_ref();
        let argc = argv.len();
        let mid = RSymbol::new(self.mrb, name).mid();
        let val =
            unsafe { minu_funcall_argv(self.mrb, self.val, mid, argc as _, argv.as_ptr() as _) };
        println!("{}", unsafe {
            String::try_from_mrb(MrbValue::new(self.mrb, minu_inspect(self.mrb, val)))?
        });
        RETVAL::try_from_mrb(Self::new(self.mrb, val))
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
