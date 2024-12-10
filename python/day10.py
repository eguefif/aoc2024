import copy

file = "d10"
with open(f"../inputs/{file}", "r") as f:
    inputs = [list(map(lambda x: int(x), list(v.strip()))) for v in f.readlines()]


def dump(inputs):
    for row in inputs:
        for v in row:
            print(str(v), end="")
        print()
    print()


dump(inputs)


def search_path(grid, x, y, counter):
    if grid[y][x] == "x":
        return counter
    if grid[y][x] == 9:
        grid[y][x] = "x"
        return counter + 1

    value = grid[y][x]
    grid[y][x] = "x"
    if y + 1 < len(grid) and grid[y + 1][x] == value + 1:
        counter = search_path(grid, x, y + 1, counter)
    if x + 1 < len(grid[0]) and grid[y][x + 1] == value + 1:
        counter = search_path(grid, x + 1, y, counter)
    if y - 1 >= 0 and grid[y - 1][x] == value + 1:
        counter = search_path(grid, x, y - 1, counter)
    if x - 1 >= 0 and grid[y][x - 1] == value + 1:
        counter = search_path(grid, x - 1, y, counter)

    return counter


def search_path2(grid, x, y, counter):
    if grid[y][x] == 9:
        return counter + 1

    value = grid[y][x]
    if y + 1 < len(grid) and grid[y + 1][x] == value + 1:
        counter = search_path2(grid, x, y + 1, counter)
    if x + 1 < len(grid[0]) and grid[y][x + 1] == value + 1:
        counter = search_path2(grid, x + 1, y, counter)
    if y - 1 >= 0 and grid[y - 1][x] == value + 1:
        counter = search_path2(grid, x, y - 1, counter)
    if x - 1 >= 0 and grid[y][x - 1] == value + 1:
        counter = search_path2(grid, x - 1, y, counter)

    return counter


def part1(grid):
    acc = 0
    for y, row in enumerate(grid):
        for x, points in enumerate(row):
            if points == 0:
                acc += search_path(copy.deepcopy(grid), x, y, 0)
    return acc


def part2(grid):
    acc = 0
    for y, row in enumerate(grid):
        for x, points in enumerate(row):
            if points == 0:
                acc += search_path2(grid, x, y, 0)
    return acc


ans1 = part1(copy.deepcopy(inputs))
ans2 = part2(inputs)
print("part1: ", ans1)
print("part2: ", ans2)

assert ans1 == 36
assert ans2 == 81
