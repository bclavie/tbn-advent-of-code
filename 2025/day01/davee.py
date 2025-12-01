dial = 50
part1_password = 0
part2_password = 0

with open('day01/input.txt', 'r') as f:
    for line in f:
        direction = line[0]
        amount = int(line[1:])

        assert(direction == 'L' or direction == 'R')
        assert(amount >= 0)

        if direction == 'L':
            zero_dist = dial if dial != 0 else 100
            dial -= amount
        else:
            zero_dist =  100 - dial if dial != 0 else 100
            dial += amount

        if amount >= zero_dist:
            part2_password += 1 + (amount - zero_dist) // 100

        dial %= 100

        if dial == 0:
            part1_password += 1
    
    print(f'part 1: {part1_password}, part 2: {part2_password}')
