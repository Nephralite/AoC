input = readlines(open("input.txt"))[1]

# original part one solution
#function part_one()
#    for i = 4:length(input)
#        if length(input[i] ∪ input[i-1] ∪ input[i-2] ∪ input[i-3]) == 4
#            return i
#        end
#    end
#end

function main(unique)
    for i = unique:length(input)
        nodups = input[i]
        for j = 1:unique-1 #not writing a 14 item union statement
            nodups = nodups ∪ input[i-j] 
        end
        if length(nodups) == unique
            return i
        end
    end
end

println(main(4))
println(main(14))
