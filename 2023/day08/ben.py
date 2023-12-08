from functools import reduce
from itertools import cycle
from math import lcm


def build_graph(raw_graph):
    graph = {}
    for entry in raw_graph:
        node, neighbors = entry.split("=")
        left, right = neighbors.strip(" ()").split(",")
        graph[node.strip()] = {"L": left.strip(), "R": right.strip()}
    return graph


def follow_instructions(instructions, graph, start, end):
    cur = start
    for step, inst in enumerate(cycle(instructions), 1):
        direction = "LR"[inst]
        if cur == end:
            return step - 1
        cur = graph[cur][direction]
    return -1


def simultaneous_nav(instructions, graph, end_suffix):
    starts = [node for node in graph if node.endswith("A")]
    steps = []
    for start in starts:
        step = 0
        current = start
        inst_index = 0
        while not current.endswith(end_suffix):
            step += 1
            direction = "LR"[instructions[inst_index % len(instructions)]]
            current = graph[current][direction]
            inst_index += 1
        steps.append(step)
    return reduce(lcm, steps, 1)


if __name__ == "__main__":
    with open("input.txt") as file:
        lines = file.read().splitlines()

    instructions = ["LR".index(x) for x in lines[0]]
    graph = build_graph(lines[2:])

    steps = follow_instructions(instructions, graph, "AAA", "ZZZ")
    print(f"Part 1: {steps}")

    lcm_steps = simultaneous_nav(instructions, graph, "Z")
    print(f"Part 2: {lcm_steps}")
