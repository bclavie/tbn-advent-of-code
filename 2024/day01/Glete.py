def part_one():
    # Ignore how scuffed this start is
    with open('2024\day01\my_input.txt', 'r') as input_file:
        full_input_list = [x for x in input_file]

    left_side, right_side = [], []

    for value in full_input_list:
        value = value.replace(' ', '')

        left_side.append(value[:-6])
        right_side.append(value[5:])

    left_sorted = sorted(int(x) for x in left_side)
    right_sorted = sorted(int(x) for x in right_side)

    total = sum(abs(left - right) for left, right in zip(left_sorted, right_sorted))

    return total, left_sorted, right_sorted

def part_two(left_sorted, right_sorted):
    return sum(left * sum(1 for right in right_sorted if right == left) for left in left_sorted)

if __name__ == "__main__":
    p1, left, right = part_one()
    p2 = part_two(left, right)
    print(f'Part 1 answer: {p1}\nPart 2 answer: {p2}')
