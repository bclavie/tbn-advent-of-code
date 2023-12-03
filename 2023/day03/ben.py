from collections import defaultdict
import math
import re
import string

parts = defaultdict(list)
board = [x.strip() for x in open("input.txt", "r").readlines()]
board_width = len(board[0])
board_height = len(board)

NON_CHARS = string.digits + "."

chars = set()

for row in range(board_height):
    for column in range(board_width):
        if board[row][column] not in NON_CHARS:
            chars.add((row, column))


for row_index, row in enumerate(board):
    for occurence in re.finditer(r"\d+", row):
        # Find the edges around the number
        edge = set()
        for row_i in (row_index - 1, row_index, row_index + 1):
            for col_i in range(occurence.start() - 1, occurence.end() + 1):
                edge.add((row_i, col_i))

        # Add the number to parts if it is adjacent to a symbol
        for inter in edge & chars:
            parts[inter].append(int(occurence.group(0)))


part_1 = sum(sum(p) for p in parts.values())
part_2 = sum(math.prod(p) for p in parts.values() if len(p) == 2)

print("Part 1:", part_1)
print("Part 2:", part_2)
