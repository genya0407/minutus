#[minutus::wrap(class_method = "new", method = "distance", method = "name_with_prefix")]
struct Point {
    x: i64,
    y: i64,
    name: String,
}

impl Point {
    #[minutus::class_method]
    pub fn new(x: i64, y: i64, name: String) -> Self {
        Self { x, y, name }
    }

    #[minutus::method]
    pub fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    #[minutus::method]
    pub fn name_with_prefix(&self, prefix: String) -> String {
        [prefix, self.name.clone()].join("_")
    }
}

minutus::extern_methods! {
    Point;
    fn name() -> String;
    fn new_2(x: i64, y: i64, name: String) -> ::minutus::data::DataPtr<Point> => "new";
    fn inspect_2(self) -> String => "inspect";
    fn to_s(self) -> String;
    fn name_with_prefix_2(self, prefix: String) -> String => "name_with_prefix";
}

use minutus::types::*;
use minutus::Evaluator;

#[test]
pub fn test_funcall() {
    let runtime = Evaluator::build();
    Point::define_class_on_mrb(runtime.mrb());
    let point = Point::try_from_mrb(runtime.evaluate("Point.new(1,2,'test')").unwrap()).unwrap();
    assert_regex_match(r"#<Point:0x[0-9a-f]+>", &point.inspect_2());
    assert_regex_match(r"#<Point:0x[0-9a-f]+>", &point.to_s());
    assert_regex_match(
        r"#<Point:0x[0-9a-f]+>",
        &Point::new_2(runtime.mrb(), 100, 200, String::from("hogeee")).to_s(),
    );
    assert_eq!("Point", &Point::name(runtime.mrb()));
}

fn assert_regex_match(pat: &str, val: &str) {
    let re = regex::Regex::new(pat).unwrap();
    assert!(re.is_match(val))
}

minutus::define_funcall! {
    fn inspect(self) -> String;
    fn inspect_2(self) -> MrbValue => "to_s";
}

#[test]
pub fn test_define_funcall() {
    let runtime = minutus::Evaluator::build();
    let retval = runtime.evaluate("123").unwrap();
    let inspected = retval.inspect();
    println!("{}", inspected);
    let inspected = retval.inspect_2();
    let inspected = inspected.inspect();
    println!("{}", inspected);
}
