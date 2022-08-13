//! Minutus is a library which enables you to 1) embed mruby into your Rust project, and 2) create mrbgem in Rust.

pub mod data;
pub mod types;

/// Define methods for [MrbValue][`types::MrbValue`].
///
/// # Examples
///
/// ```
/// minutus::define_funcall! {
///     fn inspect(self) -> String;
///     fn concat(self, other: Vec<&str>) -> Vec<String> => "+";
/// }
///
/// fn main() {
///     let runtime = minutus::build_simple_evaluator();
///     let mruby_array: minutus::types::MrbValue = runtime.evaluate("['aaa', 'bbb']").unwrap();
///     assert_eq!("[\"aaa\", \"bbb\"]", mruby_array.inspect());
///     assert_eq!(vec![String::from("aaa"), String::from("bbb"), String::from("ccc")], mruby_array.concat(vec!["ccc"]));
/// }
/// ```
pub use minutus_macros::define_funcall;

/// Define methods for [DataPtr][`data::DataPtr`].
///
/// # Examples
///
/// ```
/// extern crate minutus;
///
/// #[minutus::wrap(class_method = "new", method = "distance")]
/// struct Point {
///     x: i64,
///     y: i64,
/// }
///
/// impl Point {
///     #[minutus::class_method]
///     pub fn new(x: i64, y: i64) -> Self {
///         Self { x, y }
///     }
///
///     #[minutus::method]
///     pub fn distance(&self, other: &Point) -> f64 {
///         (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
///     }
/// }
///
/// minutus::extern_methods! {
///     Point;
///     fn name() -> String; // class method
///     fn inspect(self) -> String; // instance method
/// }
///
/// fn main() {
///     use minutus::types::FromMrb; // for using `Point::from_mrb`
///
///     let runtime = minutus::Evaluator::build(Point::define_class_on_mrb, Point::from_mrb);
///
///     let point = runtime.evaluate("Point.new(1,2)").unwrap();
///     // evaluates `point.inspect` in mruby world, and returns its value
///     point.inspect(); // => "#<Point:0x140009fb0>"
///
///     // evaluates `Point.name` in mruby world, and returns its value
///     // note: class methods requires `mrb` as argument
///     Point::name(runtime.mrb()); // => "Point"
/// }
/// ```
pub use minutus_macros::extern_methods;

/// Macro that generates [RData](https://mruby.org/docs/api/headers/mruby_2Fdata.h.html) definition for Rust types.
///
/// # Example
///
/// See also [`data`] and [`extern_methods!`]
///
/// ```
/// extern crate minutus;
///
/// // You can repeat `class_method` and `method` any times.
/// #[minutus::wrap(class_method = "new", method = "distance")]
/// struct Point {
///     x: i64,
///     y: i64,
/// }
///
/// impl Point {
///     #[minutus::class_method]
///     pub fn new(x: i64, y: i64) -> Self {
///         Self { x, y }
///     }
///
///     #[minutus::method]
///     pub fn distance(&self, other: &Point) -> f64 {
///         (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
///     }
/// }
///
/// fn main() {
///     use minutus::types::FromMrb; // for using `Point::from_mrb`
///
///     let runtime = minutus::build_simple_evaluator();
///
///     // register class in mruby
///     Point::define_class_on_mrb(runtime.mrb());
///
///     runtime.evaluate(
///       "
///         p1 = Point.new(1,1)
///         p2 = Point.new(2,2)
///         p p1.distance(p2) # => prints 1.41421356
///       "
///     ).unwrap();
/// }
/// ```
pub use minutus_macros::wrap;

/// See [`wrap`]
pub use minutus_macros::{class_method, method, MrbData};

mod evaluator;
mod utils;
pub use evaluator::*;
pub mod mruby;

pub use minutus_mruby_build_utils::MRubyManager;
