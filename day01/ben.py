import re
from functools import reduce
print("Part 1:", sum([int(f"{digits[0]}{digits[-1]}") 
for line in open("input.txt", "r") for digits in 
[re.findall(r"\d", line)]]))
print("Part 2:", sum(map(lambda line: int((digits := 
re.findall(r"(\d)", reduce(lambda l, nw: 
l.replace(nw[0], nw[1]), [(n, n + str(i + 1) + n) for 
i, n in enumerate(["one", "two", "three", "four", 
"five", "six", "seven", "eight", "nine"])] 
,line,)))[0] + digits[-1]), open("input.txt"))))

