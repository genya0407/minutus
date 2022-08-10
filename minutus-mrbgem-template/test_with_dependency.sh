set -eux

cargo build
CMD=$(cargo metadata --format-version 1 | jq -r .target_directory)/debug/minutus-mrbgem-template

NAME=mruby-polars
cd tmp
rm -rf $NAME

MRBGEM_TEMPLATE_DEBUG=true $CMD -u 'somegithubuser' -a 'Some Author' $NAME
cd $NAME

cargo add polars@0.23.1
cat <<CODE > src/lib.rs
use polars::prelude::*;
use std::cell::RefCell;

#[minutus::wrap(method = "add_integer", method = "build")]
#[derive(Clone)]
pub struct DataFrameBuilder(RefCell<Vec<Series>>);

impl DataFrameBuilder {
    #[minutus::method]
    pub fn add_integer(&self, name: String, data: Vec<i64>) -> Self {
        self.0.borrow_mut().push(Series::new(&name, &data));
        self.clone()
    }

    #[minutus::method]
    pub fn build(&self) -> DF {
        DF {
            df: DataFrame::new(self.0.borrow().to_vec()).unwrap(),
        }
    }
}

#[minutus::wrap(method = "inspect", class_method = "builder", method = "select")]
pub struct DF {
    df: DataFrame,
}

impl DF {
    #[minutus::class_method]
    pub fn builder() -> DataFrameBuilder {
        DataFrameBuilder(RefCell::new(vec![]))
    }

    #[minutus::method]
    pub fn inspect(&self) -> String {
        self.df.to_string()
    }

    #[minutus::method]
    pub fn select(&self, names: Vec<String>) -> Self {
        Self {
            df: self.df.select(names).unwrap(),
        }
    }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_polars_gem_init(mrb: *mut minutus::mruby::minu_state) {
    DataFrameBuilder::define_class_on_mrb(mrb);
    DF::define_class_on_mrb(mrb)
}

#[no_mangle]
pub extern "C" fn mrb_mruby_polars_gem_final(_mrb: *mut minutus::mruby::minu_state) {}
CODE

cat <<RB > mrblib/mrb_polars.rb
class DF
  def self.from(hash)
    b = builder
    hash.each do |k, v|
      case v.first
      when Fixnum
        b.add_integer(k, v)
      else
        raise "Unexpected type: #{v.first.class}"
      end
    end
    b.build
  end
end
RB

cat <<TEST > test/mrb_polars.rb
##
## Polars Test
##

assert("DF") do
  df = DF.builder.add_integer("age", [1,2,3]).add_integer("height", [111,1222,333]).build
  assert_equal(df.inspect,<<~RESULT.chomp)
    shape: (3, 2)
    ┌─────┬────────┐
    │ age ┆ height │
    │ --- ┆ ---    │
    │ i64 ┆ i64    │
    ╞═════╪════════╡
    │ 1   ┆ 111    │
    ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
    │ 2   ┆ 1222   │
    ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
    │ 3   ┆ 333    │
    └─────┴────────┘
  RESULT

  assert_equal(df.select(["height"]).inspect, <<~RESULT.chomp)
    shape: (3, 1)
    ┌────────┐
    │ height │
    │ ---    │
    │ i64    │
    ╞════════╡
    │ 111    │
    ├╌╌╌╌╌╌╌╌┤
    │ 1222   │
    ├╌╌╌╌╌╌╌╌┤
    │ 333    │
    └────────┘
  RESULT
end

assert("DF#from") do
  df = DF.from(
    "age" => [1,2,3],
    "height" => [111,1222,333]
  )

  assert_equal(df.inspect,<<~RESULT.chomp)
    shape: (3, 2)
    ┌─────┬────────┐
    │ age ┆ height │
    │ --- ┆ ---    │
    │ i64 ┆ i64    │
    ╞═════╪════════╡
    │ 1   ┆ 111    │
    ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
    │ 2   ┆ 1222   │
    ├╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
    │ 3   ┆ 333    │
    └─────┴────────┘
  RESULT

  assert_equal(df.select(["height"]).inspect, <<~RESULT.chomp)
    shape: (3, 1)
    ┌────────┐
    │ height │
    │ ---    │
    │ i64    │
    ╞════════╡
    │ 111    │
    ├╌╌╌╌╌╌╌╌┤
    │ 1222   │
    ├╌╌╌╌╌╌╌╌┤
    │ 333    │
    └────────┘
  RESULT
end
TEST

rake test
