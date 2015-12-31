def first_differing_bit(a,b)
  (a^b).bit_length - 1
end
def xort(ary)
  required_bits = []
  ary.each_cons(2) do |a,b|
    i = first_differing_bit(a,b)
    if i > -1
      bit = a[i]
      puts [a,b, i, bit, required_bits[i]].join(",")
      if required_bits[i] && required_bits[i] != bit
        puts required_bits
        return -1
      else
        required_bits[i] = bit
      end
    end
  end
  required_bits.map(&:to_i).reverse.join.to_i(2)
end
xort([4, 7, 6, 1, 0, 3])
puts "next"
xort([4, 7, 1, 6, 0, 3])
