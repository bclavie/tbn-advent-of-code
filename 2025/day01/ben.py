def parse_line(line):
    return (-1 if line[0] == 'L' else 1, int(line[1:]))

def counter(moves):
    curr, count = 0, 0
    for step in moves:
        curr += step
        count += 1 if curr % 100 == 0 else 0
    return count

def part_1(instructions):
    p1 = [50]
    for direction, distance in instructions:
        p1.append(direction * distance)
    return counter(p1)

def part_2(instructions):
    p2 = [50]
    for direction, distance in instructions:
        p2.extend([direction] * distance)
    return counter(p2)

if __name__ == '__main__':
    with open('input.txt') as f:
        instructions = [parse_line(line.strip()) for line in f if line.strip()]
    print(f'Part 1: {part_1(instructions)}')
    print(f'Part 2: {part_2(instructions)}')