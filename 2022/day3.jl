function score(i::Int)
    return (i > 90) ? i - 96 : i - 38
end

function part_one()
    input = readlines(open("input.txt"))
    priorities = 0
    for line in input
        overlap = (line[begin:length(line)÷2] ∩ line[length(line)÷2+1:end])[1] 
        priorities += score(Int(overlap))
    end
    return priorities
end

function part_two()
    input = readlines(open("input.txt"))
    priorities = 0
    for i = 0:length(input)÷3-1
        overlap = (input[3*i+1] ∩ input[3*i+2] ∩ input[3*i+3])[1]
        priorities += score(Int(overlap))
    end
    return priorities
end

println(part_one())
println(part_two())
