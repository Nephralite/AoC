function make_stack(input)
    stack::Vector{Vector{Char}} = []
    for i = 1:9
        inner::Vector{Char} = []
        for j = 1:8
            if input[9-j][4i-2] != ' '
                push!(inner, input[9-j][4i-2])
            end
        end
        push!(stack, inner)
    end
    return stack
end

function move!(n, from, to, stack)
    for i = 1:n
        push!(stack[to], pop!(stack[from]))
    end
end

function main(f)
    input = readlines(open("input.txt"))
    stack = make_stack(input)
    #println(stack)
    for line in input[11:end]
        nums= split(line, !isnumeric, keepempty=false)
        #print(stack)
        #println(nums)
        f(parse(Int,nums[1]), parse(Int,nums[2]), parse(Int,nums[3]), stack)
    end
    for col in stack
        print(col[end])
    end
    println("")
end

function move9001!(n, from, to, stack)
    temp = [] # instead of being intelligent, just pop it twice!
    for i = 1:n
        push!(temp, pop!(stack[from]))
    end
    while temp != []
        push!(stack[to], pop!(temp))
    end
end

main(move!)
main(move9001!)
