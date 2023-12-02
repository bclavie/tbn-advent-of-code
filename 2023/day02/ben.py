import math
import re
from collections import defaultdict


def solve(lines: list[str], part1_limits: dict[str, int]) -> tuple(int, int):
    part1_total = 0
    part2_powers = 0
    for game_id, line in enumerate(lines):
        bag = defaultdict(int)
        for num, col in re.findall(r"(\d+) (\w+)", line):
            bag[col] = max(bag[col], int(num))
        if not any(bag[col] > part1_limits[col] for col in part1_limits):
            part1_total += game_id + 1
        part2_powers += math.prod(bag.values())

    return part1_total, part2_powers


if __name__ == "__main__":
    part1, part2 = solve(
        lines=open("input.txt", "r").readlines(),
        part1_limits={"red": 12, "green": 13, "blue": 14},
    )
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")
