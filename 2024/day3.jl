input = read("input.txt", String)
 
function part_one(input)
    matches = collect(eachmatch(r"mul\((\d{1,3}),(\d{1,3})\)", input))
    cleaned = map(m -> parse.(Int64, m.captures), matches)
    return sum(prod.(cleaned))
 end
 function part_two(input)
     cleaned = replace(input, r"don\'t\(\).*?(?:do\(\)|$)" => "")
     return part_one(cleaned)
 end
 println(part_one(input))
 println(part_two(input))
