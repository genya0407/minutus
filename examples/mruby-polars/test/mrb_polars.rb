##
## Polars Test
##

assert("DF") do
  df = DF.builder.add_integer("age", [1,2,3]).add_integer("height", [111,1222,333]).build
  assert_equal(df.to_s,<<~RESULT.chomp)
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

  assert_equal(df.select(["height"]).to_s, <<~RESULT.chomp)
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

  assert_equal(df.to_s,<<~RESULT.chomp)
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

  assert_equal(df.select(["height"]).to_s, <<~RESULT.chomp)
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
