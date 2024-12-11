content = "28591 78 0 3159881 4254 524155 598 1"
inputs = [int(v) for v in content.strip().split()]
inputs = [125, 17]


def part1(stones, blinks=25):
    for _ in range(0, blinks):
        new_stones = []
        for stone in stones:
            if stone == 0:
                new_stones.append(1)
            elif len(str(stone)) % 2 == 0:
                st = str(stone)
                mid = int(len(st) / 2)
                a = st[:mid]
                b = st[mid:]
                new_stones.append(int(a))
                new_stones.append(int(b))
            else:
                new_stones.append(stone * 2024)
        stones = new_stones
    return len(stones)


ans1 = part1(inputs.copy())
ans2 = part1(inputs.copy(), 75)

print("part1: ", ans1)
print("part2: ", ans2)
assert ans1 == 55312
