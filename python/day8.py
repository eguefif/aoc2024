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


def is_on_line(xa, ya, slope, intercept, vertical_line, x):
    if vertical_line:
        return xa == x
    return ya == xa * slope + intercept


def check2(xa, ya, x, y, grid):
    vertical_line = False
    if xa - x == 0:
        vertical_line = True
        slope = 0
    else:
        slope = (ya - y) / (xa - x)
    intercept = y - slope * x

    sym = grid[ya][xa]
    for xa2, row in enumerate(grid):
        for ya2, _ in enumerate(row):
            if ya2 == ya and xa2 == xa or xa2 == x and xa2 == x:
                continue
            if grid[ya2][xa2] != sym:
                continue
            if is_on_line(xa2, ya2, slope, intercept, vertical_line, x):
                return True
    return False


def is_antinode2(x, y, grid):
    for ya, row in enumerate(grid):
        for xa, _ in enumerate(row):
            if grid[ya][xa] != ".":
                if check2(xa, ya, x, y, grid):
                    return True

    return False


def is_not_uniq(grid, sym):
    for row in grid:
        for s in row:
            if sym == s:
                return True
    return False


def count_sym(grid):
    syms = set()
    acc = 0
    for row in grid:
        for sym in row:
            if sym != ".":
                if sym in syms:
                    acc += 1
                else:
                    syms.add(sym)

    return acc + len(syms)


def part2(grid):
    for y, row in enumerate(grid):
        for x, s in enumerate(row):
            if is_antinode2(x, y, grid) and s == ".":
                points.add((x, y))

    counts = count_sym(grid)
    print(f"{len(points)} + {counts}")
    return len(points) + counts


ans = part1(inputs.copy())
ans2 = part2(inputs.copy())
print(points)
print_grid()
print("part 1: ", ans)
print("part 2: ", ans2)
assert ans == 14
assert ans2 == 34
