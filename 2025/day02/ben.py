import bisect

def parse_lines(lines):
    return [[int(k) for k in x.split("-")] for x in lines.split(",")]    

def process(inputs):
    limit = max(r[1] for r in inputs)
    len_limit = len(str(limit))
    p1, p2 = set(), set()
    for L in range(1, len_limit // 2 + 1):
        for base in range(10**(L-1), 10**L):
            s = str(base)
            val_s = s * 2
            while len(val_s) <= len_limit:
                val = int(val_s)
                if val <= limit:
                    if len(val_s) == 2 * L:
                        p1.add(val) 
                    p2.add(val)                        
                val_s += s

    return sorted(p1), sorted(p2)

def tally(part, inputs):
    return sum(sum(n for n in part[bisect.bisect_left(part, x):bisect.bisect_right(part, y)]) for x, y in inputs)

if __name__ == "__main__":
    with open("input.txt") as f:
        inputs = parse_lines(f.read())
    p1, p2 = process(inputs)
    print(f"Part 1: {tally(p1, inputs)}")
    print(f"Part 2: {tally(p2, inputs)}")