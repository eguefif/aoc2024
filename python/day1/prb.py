left = []
right = []
with open("./input", "r") as f:
    lines = f.readlines()
    for line in lines:
        v1, v2 = line.strip().split("   ")
        left.append(int(v1))
        right.append(int(v2))


def part1(v1, v2):
    acc = 0
    acc = sum((abs(a - b) for a, b in zip(v1, v2)))
    return acc


def part2(left, right):
    acc = sum((a * right.count(a) for a in left))
    return acc


p1 = part1(sorted(left), sorted(right))
p2 = part2(left, right)

print(f"part1: {p1}")
print(f"part2: {p2}")
