use polars::prelude::*;
use std::cell::RefCell;

#[minutus::wrap(method = "add_integer", method = "build")]
#[derive(Clone)]
struct DataFrameBuilder(RefCell<Vec<Series>>);

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

#[minutus::wrap(method = "to_s", class_method = "builder", method = "select")]
struct DF {
    df: DataFrame,
}

impl DF {
    #[minutus::class_method]
    pub fn builder() -> DataFrameBuilder {
        DataFrameBuilder(RefCell::new(vec![]))
    }

    #[minutus::method]
    pub fn to_s(&self) -> String {
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
