input = [parse.(Int, split(line)) for line in readlines(open("input.txt"))]

function safe(list; tol=false)
    valid = all(-3 .<= diff(list) .< 0) or all(0 .< diff(list) .<= 3)
    valid && return true
    !tol && return valid
    for i in 1:length(list)
        safe(list[Not(i)]) && return true
    end
    return false
end

function part_one()
    return sum(safe.(input))
end
function part_two()
    return sum(safe.(input, tol=true))
end
