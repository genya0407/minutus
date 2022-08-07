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
