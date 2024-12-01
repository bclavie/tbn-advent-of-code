def day1a(): return sum(abs(x - y) for x, y in zip(sorted(data[0::2]), sorted(data[1::2])))
def day1b(): return sum(a * sorted(data[1::2]).count(a) for a in sorted(data[0::2]))

if __name__ == "__main__":
    data = [int(x) for x in open('input/day01.txt').read().split()]
    print("Part 1:", day1a())
    print("Part 2:", day1b())