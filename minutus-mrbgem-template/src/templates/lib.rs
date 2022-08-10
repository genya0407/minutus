#[minutus::wrap(class_method = "new", method = "distance", method = "name_with_prefix")]
struct {{ class_name }} {
    x: i64,
    y: i64,
    name: String,
}

impl {{ class_name }} {
    #[minutus::class_method]
    pub fn new(x: i64, y: i64, name: String) -> Self {
        Self { x, y, name }
    }

    #[minutus::method]
    pub fn distance(&self, other: &{{ class_name }}) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    #[minutus::method]
    pub fn name_with_prefix(&self, prefix: String) -> String {
        [prefix, self.name.clone()].join("_")
    }
}

#[no_mangle]
pub extern "C" fn mrb_{{ underscored_package_name }}_gem_init(mrb: *mut minutus::mruby::minu_state) {
    {{ class_name }}::define_class_on_mrb(mrb);
}

#[no_mangle]
pub extern "C" fn mrb_{{ underscored_package_name }}_gem_final(_mrb: *mut minutus::mruby::minu_state) {}        
