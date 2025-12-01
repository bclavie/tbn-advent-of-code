def crank_that_safe():
    part1_result, part2_result = 0, 0
    with open("day1/input.txt", "r") as f:
        position = 50
        for x in f:
            direction = x[0]
            value = int(x[1:])
            starting_position = position
            wraparounds = 0

            for _ in range(value):
                if direction == "R":
                    position = (position + 1) % 100
                else:
                    position = (position - 1) % 100

                if position == 0:
                    wraparounds += 1

            position = (
                (starting_position + value) % 100
                if direction == "R"
                else (starting_position - value) % 100
            )

            if position == 0:
                part1_result += 1

            part2_result += wraparounds
    return part1_result, part2_result


if __name__ == "__main__":
    part1_result, part2_result = crank_that_safe()
    print(part1_result, part2_result)
