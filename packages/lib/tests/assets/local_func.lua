local function printNumberPlus10(n)
    print(n + 10);
end

local function printNumberPlus20(n)
    print(n + 20);
end

function GlobalFunction(n)
    printNumberPlus10(n);
    printNumberPlus20(n);
end