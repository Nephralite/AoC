function part_one()
    input = readlines(open("input.txt"))
    subsets = 0
    for line in input
        nums = split(line, !isnumeric)
        set1 = [parse(Int, nums[1]):parse(Int, nums[2]);]
        set2 = [parse(Int, nums[3]):parse(Int, nums[4]);]
        subsets += (issubset(set1, set2) | issubset(set2, set1)) ? 1 : 0
    end
    return subsets
end

function part_two()
    input = readlines(open("input.txt"))
    subsets = 0
    for line in input
        nums = split(line, !isnumeric)
        set1 = [parse(Int, nums[1]):parse(Int, nums[2]);]
        set2 = [parse(Int, nums[3]):parse(Int, nums[4]);]
        subsets += ((set1 ∩ set2) != []) ? 1 : 0
    end
    return subsets
end

println(part_one())
println(part_two())
