with open("../inputs/d7", "r") as f:
    content = f.readlines()


def get_input(content):
    values = []
    ops = []
    for line in content:
        splits = line.split(":")
        values.append(int(splits[0].strip()))
        ops.append([int(v.strip()) for v in splits[1].split(" ") if len(v) > 0])
    return (values, ops)


def calculate(value, op, result, index):
    if index == len(op):
        return result
    check = calculate(value, op, result + op[index], index + 1)
    if check == value:
        return value

    check = calculate(value, op, result * op[index], index + 1)
    if check == value:
        return value
    return 0


def part1(content):
    values, ops = get_input(content)

    acc = 0
    for value, op in zip(values, ops):
        res = calculate(value, op, op[0], 1)
        acc += res
    return acc


def combine(v1, v2):
    return int(str(v1) + str(v2))


def calculate2(value, op, result, index):
    if index == len(op):
        return result
    check = calculate2(value, op, result + op[index], index + 1)
    if check == value:
        return value

    check = calculate2(value, op, result * op[index], index + 1)
    if check == value:
        return value

    check = calculate2(value, op, combine(result, op[index]), index + 1)
    if check == value:
        return value
    return 0


def part2(content):
    values, ops = get_input(content)
    acc = 0

    for value, op in zip(values, ops):
        res = calculate2(value, op, op[0], 1)
        acc += res
    return acc


ans = part1(content)
ans2 = part2(content)
print("part1: ", ans)
print("part2: ", ans2)

assert ans2 == 11387
