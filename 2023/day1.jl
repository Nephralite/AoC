input = readlines(open("input.txt"))

function part_one(input)
    return 
end

function part_two()
    filtered = map(line -> replace(line, "oneight"=>"18", "twone"=>"21", "threeight"=>"38", "fiveight"=>"58", "sevenine"=>"79", "eightwo"=>"82", "eighthree"=>"83", "nineight"=>"98", "one"=>"1", "two"=>"2", "three"=>"3","four"=>"4","five"=>"5","six"=>"6","seven"=>"7","eight"=>"8","nine"=>"9"), input)
    return part_one(filtered)
end

println(part_one(input))
println(part_two())
