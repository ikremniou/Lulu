function Fibonachi(n)
    if n < 2 then
        return 1
    else
        return Fibonachi(n - 1) + Fibonachi(n - 2)
    end
end