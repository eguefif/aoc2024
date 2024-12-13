content = "28591 78 0 3159881 4254 524155 598 1"
inputs = [int(v) for v in content.strip().split()]
# inputs = [125, 17]


def is_pair(stone):
    acc = 1
    div = int(stone / 10)
    while div > 0:
        div = int(div / 10)
        acc += 1
    return acc


def get_pair(stone):
    div = 10 ** int(is_pair(stone) / 2)
    first = int(stone % div)
    second = int(stone / div)
    return first, second


def get_stones(st, blinks):
    acc = 0
    if blinks == 0:
        return 1
    if st == 0:
        acc += get_stones(1, blinks - 1)
    elif is_pair(st) % 2 == 0:
        p1, p2 = get_pair(st)
        acc += get_stones(p1, blinks - 1)
        acc += get_stones(p2, blinks - 1)
    else:
        acc += get_stones(st * 2024, blinks - 1)

    return acc


def part1(stones, blinks=25):
    acc = 0
    for stone in stones:
        acc += get_stones(stone, blinks)

    return acc


ans1 = part1(inputs.copy())
ans2 = part1(inputs.copy(), 75)

print("part1: ", ans1)
# print("part2: ", ans2)
assert ans1 == 220722
