from itertools import pairwise

with open('2024\day02\my_input.txt', 'r') as input_file:
    full_input_list = [[int(x) for x in line.split()] for line in input_file]

result = 0
part2_result = 0

modified_input_list = full_input_list.copy()

leftover_list = []

for count, value in enumerate(full_input_list):
    for a, b in pairwise(value):
        difference = b - a
        if abs(difference) > 3 or abs(difference) == 0:
            modified_input_list.remove(value)
            leftover_list.append(value)
            break

result = [None] * len(modified_input_list)
part2_result = [None] * len(full_input_list)

def safety_check(count, value, part2=None):
    safety = ''
    for index, (a, b) in enumerate(pairwise(value)):
        if a > b:
            if safety == '':
                safety = 'decreasing'
            elif safety == 'increasing':
                safety = 'unsafe'
                if not part2:
                    leftover_list.append(value)
        elif a < b:
            if safety == '':
                safety = 'increasing'
            elif safety == 'decreasing':
                safety = 'unsafe'
                if not part2:
                    leftover_list.append(value)
                
        if index == len(value) - 2:
            if safety != 'unsafe':
                safety = 'safe'
            if not part2:
                result[count] = safety
            else:
                part2_result[count] = safety

for count, value in enumerate(modified_input_list):
    safety_check(count, value)
                     
part1_result_count = 0

for count, value in enumerate(result):
    if value != 'unsafe':
        part1_result_count += 1

def part_2(leftovers):
    for count, value in enumerate(leftovers):
        for a, b in pairwise(value):
            difference = b - a
            if abs(difference) > 3 or abs(difference) == 0:
                value.remove(b)
                break
        safety_check(count, value, part2=True)

part_2(leftover_list)

part2_result_count = sum(1 for x in part2_result if x == 'safe')

print(f'Part 1 answer: {part1_result_count}\nPart 2 answer: {part1_result_count + part2_result_count}')

# Its joever
