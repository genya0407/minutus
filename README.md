# Minutus

[![minutus](https://img.shields.io/crates/v/minutus.svg)](https://crates.io/crates/minutus)
![ci status](https://github.com/genya0407/minutus/actions/workflows/test.yml/badge.svg)
![license](https://img.shields.io/github/license/genya0407/minutus)

Heavily inspired by [Magnus](https://github.com/matsadler/magnus).

By minutus, you can do:

- [Embed mruby in Rust](#embed-mruby-in-rust)
- [Create mrbgem by Rust](#create-mrbgem-by-rust)
- [Connect mruby's class and Rust's struct](#connect-mrubys-class-and-rusts-struct)

## Embed mruby in Rust

First, add minutus to your crate's dependencies.

```shell-session
cargo add minutus --features mruby_3_1_0,link_mruby
```

Then, write code like:

```rust
// src/main.rs

// This enables you to call `some_method` from Rust world.
minutus::define_funcall!{
  fn some_method(self, arr: Vec<i64>) -> i64;
}

fn main() {
    let runtime = minutus::build_simple_evaluator();

    // define `some_method` in mruby world
    runtime.evaluate(
      "
      def some_method(arr)
        p arr
        arr.reduce(&:+)
      end
      "
    ).unwrap();

    // capture mruby's main object
    let main = runtime.evaluate("self").unwrap();

    // call `some_method` on main object from Rust world.
    // in / out values are type-casted, according to `define_funcall!` definition
    let retval: i64 = main.some_method(vec![1,2,3,4]);
    println!("retval is {}", retval);
}
```

And now, you can run your code:

```shell-session
$ cargo run
...
[1, 2, 3, 4]
retval is 10
```

If you want to use custom `build_config.rb` (e.g. for using mrbgems), you have to:

1. Write your custom `build_config.rb`
2. Write `build.rs`

Minutus provides helpers for this purpose. See [examples/custom-mruby](/examples/custom-mruby).

## Create mrbgem by Rust

Install `minutus-mrbgem-template`:

```shell-session
cargo install minutus-mrbgem-template
```

Initialize mrbgem:

```shell-session
$ minutus-mrbgem-template mruby-test-mrbgem
$ cd mruby-test-mrbgem
$ tree
.
├── Cargo.toml
├── LICENSE
├── README.md
├── Rakefile
├── mrbgem.rake
├── mrblib
│   └── mrb_testmrbgem.rb
├── mruby-test-mrbgem.gem
├── src
│   ├── dummy.c
│   └── lib.rs
└── test
    └── mrb_test_mrbgem.rb
```

Now, you can build and test mrbgem.

```shell-session
$ rake test
...
  Total: 1456
     OK: 1449
     KO: 0
  Crash: 0
Warning: 0
   Skip: 7
   Time: 0.06 seconds
```

## Connect mruby's class and Rust's struct

You can bind Rust's struct with mruby's class.

If you generate mrbgem by `minutus-mrbgem-template`, `src/lib.rs` includes an example.

```rust
// src/lib.rs

/*
  `minutus::wrap` does:

  - Define mruby class. In this example, `TestMrbgem` class is defined.
  - Define bind methods to the class.
    - The functions must be marked by `class_method` macro or `method` macro.
*/
#[minutus::wrap(class_method = "new", method = "distance", method = "name_with_prefix")]
struct TestMrbgem {
    x: i64,
    y: i64,
    name: String,
}

impl TestMrbgem {
    // `minutus::class_method` marks the function as class method
    #[minutus::class_method]
    pub fn new(x: i64, y: i64, name: String) -> Self {
        Self { x, y, name }
    }

    // `minutus::class_method` marks the function as instance method
    #[minutus::method]
    pub fn distance(&self, other: &TestMrbgem) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    #[minutus::method]
    pub fn name_with_prefix(&self, prefix: String) -> String {
        [prefix, self.name.clone()].join("_")
    }
}

// These functions are recognized by mruby and executed when this mrbgem is loaded.
#[no_mangle]
pub extern "C" fn mrb_mruby_test_mrbgem_gem_init(mrb: *mut minutus::mruby::minu_state) {
    // You must call `define_class_on_mrb` here to have mruby recognize the class.
    TestMrbgem::define_class_on_mrb(mrb);
}

#[no_mangle]
pub extern "C" fn mrb_mruby_test_mrbgem_gem_final(_mrb: *mut minutus::mruby::minu_state) {}
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

Currently, only [3.1.0](https://github.com/mruby/mruby/releases/tag/3.1.0) is supported.
You can use `mruby_master` feature, but it is not guaranteed to work.

## Naming

Minutus is an antonym of [Magnus](https://github.com/matsadler/magnus),
which means _small_.
