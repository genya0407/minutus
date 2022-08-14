#[minutus::wrap(class_method = "new", method = "distance", method = "name_with_prefix")]
#[derive(Eq, PartialEq, Debug)]
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

#[test]
fn test() {
    use minutus::types::*;
    let executor = minutus::Evaluator::build();
    Point::define_class_on_mrb(executor.mrb());
    let point = Point::try_from_mrb(executor.evaluate("Point.new(1,2, 'dummy')").unwrap()).unwrap();
    assert_eq!(*point, Point::new(1, 2, String::from("dummy")))
}

#[test]
fn stress() {
    use minutus::types::*;
    let executor = minutus::Evaluator::build();
    Point::define_class_on_mrb(executor.mrb());
    let point = Point::try_from_mrb(executor.evaluate("Point.new(1,2, 'dummy')").unwrap()).unwrap();
    for _ in 0..1000 {
        executor
            .evaluate("GC.start; Point.new(1,2, 'dummy')")
            .unwrap();
    }
    assert_eq!(*point, Point::new(1, 2, String::from("dummy")))
}
