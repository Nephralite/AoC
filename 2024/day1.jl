module AOCD1
using StatsBase

input = readlines(open("input.txt"))
lists = map(line -> split(line, "   "), input)
list_one = sort(map(i -> parse(Int64, i[1]), lists))
list_two = sort(map(i -> parse(Int64, i[2]), lists))

function part_one()
  return foldl(+, map(i -> abs(list_one[i] - list_two[i]), 1:1000), init=0)
end

function part_two()
  return foldl((i,p) -> i + p[1] * p[2], pairs(filter(p -> p[1] in list_one, countmap(list_two))), init=0)
end

end # module AOCD1
