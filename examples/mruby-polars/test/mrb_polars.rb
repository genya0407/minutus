##
## Polars Test
##

assert("Polars#hello") do
  t = Polars.new "hello"
  assert_equal("hello", t.hello)
end

assert("Polars#bye") do
  t = Polars.new "hello"
  assert_equal("hello bye", t.bye)
end

assert("Polars.hi") do
  assert_equal("hi!!", Polars.hi)
end
