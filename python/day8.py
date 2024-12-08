with open("../inputs/d8", "r") as f:
    content = f.readlines()

ans = 0
inputs = [list(row.strip()) for row in content]
points = set()


def print_grid():
    for y, row in enumerate(inputs):
        for x, p in enumerate(row):
            if (x, y) in points and p == ".":
                print("#", end="")
            else:
                print(p, end="")
        print()


def check(xa, ya, x, y, grid):
    dx = xa - x
    dy = ya - y
    xa2 = x + 2 * dx
    ya2 = y + 2 * dy

    if ya2 < 0 or xa2 < 0 or ya2 >= len(grid) or xa2 >= len(grid[0]):
        return False
    if grid[ya][xa] == grid[ya2][xa2] and (dy != 0 and dx != 0):
        return True
    return False


def is_antinode(x, y, grid):
    for ya, row in enumerate(grid):
        for xa, _ in enumerate(row):
            if grid[ya][xa] != ".":
                if check(xa, ya, x, y, grid):
                    return True

    return False


def part1(grid):
    acc = 0
    for y, row in enumerate(grid):
        for x, _ in enumerate(row):
            if is_antinode(x, y, grid):
                acc += 1

    return acc


def in_grid(x, y, grid):
    return x in range(0, len(grid[0])) and y in range(len(grid))


def get_uniq_pairs(grid):
    pairs = set()
    for y, row in enumerate(grid):
        for x, v in enumerate(row):
            if v != ".":
                for y2, row2 in enumerate(grid):
                    for x2, v2 in enumerate(row2):
                        if y2 * x2 < y * x:
                            continue
                        if v2 != "." and (x, y) != (x2, y2) and v2 == v:
                            pairs.add(((x, y), (x2, y2)))
    return pairs


def part2(grid):
    pairs = get_uniq_pairs(grid)
    width = len(grid[0])
    height = len(grid)

    max_iterations = max(width, height)
    for pair in pairs:
        p1 = pair[0]
        p2 = pair[1]

        points.add(p1)
        points.add(p2)

        slope = [p2[1] - p1[1], p2[0] - p1[0]]

        for i in range(1, max_iterations):
            x = p1[0] - i * slope[1]
            y = p1[1] - i * slope[0]
            if not in_grid(x, y, grid):
                break
            points.add((x, y))

        for i in range(1, max_iterations):
            x = p1[0] + i * slope[1]
            y = p1[1] + i * slope[0]
            if not in_grid(x, y, grid):
                break
            points.add((x, y))
    return len(points)


ans = part1(inputs.copy())
ans2 = part2(inputs.copy())
print(points)
print_grid()
print("part 1: ", ans)
print("part 2: ", ans2)
assert ans == 305
assert ans2 == 1150
