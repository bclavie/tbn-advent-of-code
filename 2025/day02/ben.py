def parse_lines(lines):
    return [[int(k) for k in x.split("-")] for x in lines.split(",")]    

def generate(a, b, w):
    L = len(str(a))
    shift = 10**(L - w)
    vals = [int(str(r) * (L // w)) for r in range(a // shift, b // shift + 1)]
    return [val for val in vals if a <= val <= b]

def process(inputs):
    p1, p2 = 0, 0
    for x, y in inputs:
        curr = x
        while curr <= y:
            L = len(str(curr))
            end = min(y, 10**L - 1)
            if L % 2 == 0:
                p1 += sum(generate(curr, end, L // 2))
            unique_vals = set()
            for base in range(1, (L // 2) + 1):
                if L % base == 0:
                    generated = generate(curr, end, base)
                    unique_vals.update(generated)
            p2 += sum(unique_vals)
            curr = end + 1
    return p1, p2

if __name__ == "__main__":
    with open("input.txt") as f:
        inputs = parse_lines(f.read())
    p1, p2 = process(inputs)
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")