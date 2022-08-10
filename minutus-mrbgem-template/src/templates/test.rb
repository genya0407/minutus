assert('{{ class_name }}') do
  distance = {{ class_name }}.new(1,1, 'aaa').distance({{ class_name }}.new(2,2, 'bbb'))
  assert_equal(1.41421356, distance.round(8))

  name_with_prefix = {{ class_name }}.new(1,2,'hoge').name_with_prefix('fuga')
  assert_equal('fuga_hoge', name_with_prefix)
end
