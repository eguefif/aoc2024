content = "28591 78 0 3159881 4254 524155 598 1"
inputs = [int(v) for v in content.strip().split()]
print(inputs)
inputs = [125, 17]


def part1(stones):
    return len(stones)


ans1 = part1(inputs.copy())

print("part1: ", ans1)
assert ans1 == 55312
