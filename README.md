# Minutus

[![minutus](https://img.shields.io/crates/v/minutus.svg)](https://crates.io/crates/minutus)
[![docs.rs](https://img.shields.io/badge/docs.rs-minutus-blue)](https://docs.rs/minutus)
![ci status](https://github.com/genya0407/minutus/actions/workflows/test.yml/badge.svg)
![license](https://img.shields.io/github/license/genya0407/minutus)

Minutus is mruby binding for Rust,
which enables you to treat mruby without writing painful boilerplates.
Heavily inspired by [Magnus](https://github.com/matsadler/magnus).

By minutus, you can easily [embed mruby in Rust](#embed-mruby-in-rust),
and [create mrbgem by Rust](#create-mrbgem-by-rust).

Minutus also provides sensible [type casting](#type-casting),
and you can [define typed functions on mruby values](#define-typed-functions-on-mruby-values)
and [wrap rust structs in mruby objects](#wrap-rust-structs-in-mruby-objects).

## Embed mruby in Rust

You can embed mruby code in Rust code.

Add minutus to your crate's dependencies with `link_mruby` feature.

```shell-session
cargo add minutus --features link_mruby
```

Write code like:

```rust
// src/main.rs

// This enables you to call `some_method` in mruby world.
minutus::define_funcall!{
  fn some_method(self, arr: Vec<i64>) -> i64;
}

fn main() {
    let runtime = minutus::Evaluator::build();

    // Define `some_method` in mruby world
    runtime.evaluate(
      "
      def some_method(arr)
        p arr
        arr.reduce(&:+)
      end
      "
    ).unwrap();

    // Capture mruby's main object
    let main = runtime.evaluate("self").unwrap();

    // Call `some_method` on main object.
    // Arguments and return values are automatically type-casted according to `define_funcall!` definition.
    let retval: i64 = main.some_method(vec![1,2,3,4]).unwrap();
    println!("retval is {}", retval);
}
```

Then, you can run your code:

```shell-session
$ cargo run
[1, 2, 3, 4] // in mruby workd
retval is 10 // in rust world
```

If you want to use custom `build_config.rb` (e.g. for using mrbgems),
you have to write custom `build.rs`

Minutus provides a helper for this purpose. See [examples/custom-mruby](/examples/custom-mruby).

## Create mrbgem by Rust

Install `minutus-mrbgem-template` and initialize mrbgem.

```shell-session
$ cargo install minutus-mrbgem-template
$ minutus-mrbgem-template mruby-example
$ tree mruby-example
mruby-example
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── Rakefile
├── mrbgem.rake
├── mrblib
│   └── mrb_example.rb
├── mruby-example.gem
├── src
│   ├── dummy.c
│   └── lib.rs
└── test
    └── mrb_example.rb
```

Now, you can build and test mrbgem.

```shell-session
$ cd mruby-example && rake test
...
  Total: 1456
     OK: 1449
     KO: 0
  Crash: 0
Warning: 0
   Skip: 7
   Time: 0.06 seconds
```

## Wrap Rust Structs in mruby Objects

You can wrap Rust's struct in mruby objects.

The following example defines `TestMrbgem` class in mruby,
which has class method `new`, and instance methods `distance` and `name_with_prefix`.

```rust
#[minutus::wrap(class_method = "new", method = "distance", method = "name_with_prefix")]
struct TestMrbgem {
    x: i64,
    y: i64,
    name: String,
}

impl TestMrbgem {
    #[minutus::class_method]
    pub fn new(x: i64, y: i64, name: String) -> Self {
        Self { x, y, name }
    }

    #[minutus::method]
    pub fn distance(&self, other: &TestMrbgem) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    #[minutus::method]
    pub fn name_with_prefix(&self, prefix: String) -> String {
        [prefix, self.name.clone()].join("_")
    }
}
```

## Define typed functions on mruby values

Use `define_funcall!` macro.

```rust
minutus::define_funcall! {
    fn inspect(self) -> String;
    fn concat(self, other: Vec<&str>) -> Vec<String> => "+";
}

fn main() {
    let runtime = minutus::Evaluator::build();

    let mruby_array = runtime.evaluate("['aaa', 'bbb']").unwrap();
    assert_eq!("[\"aaa\", \"bbb\"]", mruby_array.inspect().unwrap());
    assert_eq!(vec![String::from("aaa"), String::from("bbb"), String::from("ccc")], mruby_array.concat(vec!["ccc"]).unwrap());
}
```

## Type casting

See [minutus/src/types](minutus/src/types) for details.

| Rust type | mruby type |
|-----------|------------|
| `i8`, `i16`, `i32`, `i64`, `isize` | `Integer` |
| `u8`, `u16`, `u32`, `u64`, `usize` | `Integer` |
| `f32`, `f64` | `Float` |
| `String` | `String` |
| `Option<T>` | `T` or `nil` |
| `(T, U)`, `(T, U, V)`, etc | `[T, U]`, `[T, U, V]`, etc |
| `Vec<T>` | `Array` |
| `std::collections::HashMap<T, U>` | `Hash` |
| `minutus::types::RSymbol` | `Symbol` |
| `bool` | any object |
| `MrbData` (structs marked by `minutus::wrap`) | corresponding class |

Any value in mruby can be cast to Rust's `bool`.
Rust's `bool` cast to mruby's `true` or `false`.

## Supported mruby versions

Following versions are supported:

* [3.1.0](https://github.com/mruby/mruby/releases/tag/3.1.0)
* [3.2.0](https://github.com/mruby/mruby/releases/tag/3.2.0)
* [3.3.0](https://github.com/mruby/mruby/releases/tag/3.3.0)

You can also use mruby's `master` branch, but it is not tested.

If the version is not specified on Cargo.toml,
the latest supported stable version is used.

```toml
[dependencies]
# Use 3.3.0
minutus = "*"

# Use 3.2.0
minutus = { version = "*", features = ["mruby_3_2_0"] }

# Use master branch
minutus = { version = "*", features = ["mruby_master"] }
```

## Naming

Minutus is an antonym of [Magnus](https://github.com/matsadler/magnus),
which means _small_.
