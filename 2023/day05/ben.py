import math

from tqdm import tqdm


def extract_data(
    filename: str = "input.txt"
) -> tuple[list[int], list[tuple[int, int]], list[list[tuple[int, int, int]]]]:
    with open(filename, "r") as f:
        raw_lines = [x.strip() for x in f.readlines()]
    # get values for p1
    values = [int(value) for value in raw_lines[0].split(": ")[1].split(" ")]
    # get ranges for p2
    ranges = [(values[2 * i], values[2 * i + 1]) for i in range(len(values) // 2)]

    mappings = []
    current_mapping = []
    for line in raw_lines[3:]:
        if line == "":
            continue
        if ":" in line:
            mappings.append(current_mapping)
            current_mapping = []
            continue
        current_mapping.append(tuple(int(x) for x in line.split(" ")))

    mappings.append(current_mapping)

    return values, ranges, mappings


def part1(
    values: list[int],
    mappings: list[list[tuple[int, int, int]]],
) -> int:
    all_locations = {}
    for seed in values:
        mapped_value = seed
        for mapping in mappings:
            for destination_start, source_start, range_length in mapping:
                if source_start <= mapped_value < source_start + range_length:
                    mapped_value = destination_start + (mapped_value - source_start)
                    break
        all_locations[mapped_value] = seed

    minimum_location = min(all_locations.keys())

    return minimum_location


def part2(
    ranges: list[tuple[int, int]],
    mappings: list[list[tuple[int, int, int]]],
) -> int:
    total_range_length = sum(length for _, length in ranges)
    average_range_length = total_range_length / len(ranges)
    step_size = int(average_range_length / 10)

    explored_search_space = {}
    for range_start, range_length in ranges:
        for seed_number in range(range_start, range_start + range_length, step_size):
            final_mapped_value = seed_number
            for mapping_rule in mappings:
                for dest_start, src_start, map_length in mapping_rule:
                    if src_start <= final_mapped_value < src_start + map_length:
                        final_mapped_value = dest_start + (
                            final_mapped_value - src_start
                        )
                        break
            explored_search_space[
                (range_start, range_start + range_length, seed_number)
            ] = final_mapped_value

    range_start, range_end, estimated_value = min(
        explored_search_space.items(), key=lambda x: x[1]
    )[0]

    print(
        f"Initial step size: {step_size}",
        f"Initial best estimate: {estimated_value}",
    )

    iteration_range = range(math.ceil(math.log10(step_size)) - 1)
    for _ in tqdm(iteration_range):
        lower_search_boundary = max(estimated_value - step_size, range_start)
        upper_search_boundary = min(estimated_value + step_size, range_end)
        step_size //= 10

        for seed in range(lower_search_boundary, upper_search_boundary, step_size):
            refined_seed_value = seed
            for mapping_rule in mappings:
                for dest_start, src_start, map_length in mapping_rule:
                    if src_start <= refined_seed_value < src_start + map_length:
                        refined_seed_value = dest_start + (
                            refined_seed_value - src_start
                        )
                        break
            explored_search_space[seed] = refined_seed_value

        estimated_value, local_minimum_location = min(
            explored_search_space.items(), key=lambda x: x[1]
        )

        print(
            f"Step size: {step_size}; Current estimate {estimated_value}; Local minimum location: {local_minimum_location}"
        )

    return local_minimum_location


if __name__ == "__main__":
    values, ranges, mappings = extract_data("input.txt")
    print(f"Part 1: {part1(values=values, mappings=mappings,)}")
    print("\n_____ Part 2 _____\n")
    print(f"Global Minimum (Estimated): {part2(ranges=ranges, mappings=mappings,)}")
