import itertools

def is_safe(report):
    pair_diffs = [lvl_a - lvl_b for lvl_a, lvl_b in itertools.pairwise(report)]
    increasing = pair_diffs[0] > 0
    for diff in pair_diffs:
        if (diff > 0) != increasing:
            return False
        if abs(diff) < 1 or abs(diff) > 3:
            return False            
    return True

def day2a(): return sum(is_safe(report) for report in data)

def is_suspiciously_safe(report):
    if is_safe(report):
        return True
    report_length = len(report)
    for level_idx in range(report_length):
        modified_report = report[:level_idx] + report[level_idx + 1:]
        if is_safe(modified_report):
            return True
    return False

def day2b(): return sum(is_suspiciously_safe(report) for report in data)

if __name__ == "__main__":
    data = [[int(y) for y in x.strip().split()] for x in open('input/day02.txt').readlines()]
    print("Part 1:", day2a())
    print("Part 2:", day2b())
