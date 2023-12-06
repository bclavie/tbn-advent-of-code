from math import sqrt


def parse_input_data(
    filename: str = "input.txt",
):
    raw_lines = [x.strip() for x in open(filename, "r").readlines()]
    part_1_times = [int(val) for val in raw_lines[0].split(":")[1].strip().split()]
    part_1_records = [int(val) for val in raw_lines[1].split(":")[1].strip().split()]
    part_1 = list(zip(part_1_times, part_1_records, strict=True))

    part_2 = [
        (
            int("".join(str(x) for x in part_1_times)),
            (int("".join(str(x) for x in part_1_records))),
        )
    ]

    return part_1, part_2


def solve_quadratic(a, b, c):
    discriminant = b**2 - 4 * a * c
    root1 = (-b + sqrt(discriminant)) / (2 * a)
    root2 = (-b - sqrt(discriminant)) / (2 * a)
    return min(root1, root2), max(root1, root2)


def find_ways_to_beat_record(races):
    total_ways = 1

    for race in races:
        t, d = race
        root1, root2 = solve_quadratic(1, -t, d)
        count = sum(1 for x in range(int(root1) + 1, int(root2) + 1))
        total_ways *= count

    return total_ways


if __name__ == "__main__":
    part_1, part_2 = parse_input_data()
    print("Part 1:", find_ways_to_beat_record(part_1))
    print("Part 2:", find_ways_to_beat_record(part_2))
