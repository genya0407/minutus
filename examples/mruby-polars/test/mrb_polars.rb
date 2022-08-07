##
## Polars Test
##

assert("Point#distance") do
  assert_equal(1.41421356, Point.new(1,1).distance(Point.new(2,2)).round(8))
end
