import re

with open("../../inputs/d3", "r") as f:
    input = f.read()
    input.replace("\n", "")


def part1(input):
    reg = r"mul\(([0-9]+),([0-9]+)\)"
    acc = 0
    for mul in re.finditer(reg, input):
        acc += int(mul.group(1)) * int(mul.group(2))
    return acc


def part2(input):
    reg = r"(?<=(^|do\(\()(.(?!don't\(\)))*)mul\((\d+),(\d+)\)"
    acc = 0
    for mul in re.finditer(reg, input):
        print(mul)
        # acc += int(mul.group(1)) * int(mul.group(2))


ans1 = part1(input)
ans2 = part2(input)
print(ans1, ans2)

assert ans1 == 163931492

assert ans2 == 76911921
