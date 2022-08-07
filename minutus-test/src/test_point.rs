use crate::test_utils::Executor;

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

fn executor() -> Executor {
    Executor {
        initializer: Point::define_class_on_mrb,
    }
}

#[test]
fn test_type_convert() {
    executor().execute(
        "
            assert_raise_error(ArgumentError) do
              Point.new(1.0, 1, '')
            end
            ",
    );
    executor().execute(
        "
            assert_raise_error(ArgumentError) do
              Point.new('aaa', 1, '')
            end
            ",
    );
    executor().execute(
        "
            point = Point.new(1,2,'aaa')
            assert_eq(point.name_with_prefix('xxx'), 'xxx_aaa')
            ",
    );
}

#[test]
fn test_distance() {
    executor().execute(
        "
            a = Point.new(1,1, '')
            b = Point.new(2,2, '')
            distance = a.distance(b)
            assert_eq(distance, Math.sqrt(2))
            ",
    )
}
