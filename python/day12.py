with open("../inputs/exemple", "r") as f:
    content = f.readlines()
    grid = [list(line.strip()) for line in content]


def dump(grid, map):
    [print("".join(line)) for line in grid]

    for value in map:
        print(f"Key {value.variety}: {value}")


class Plot:
    def __init__(self, variety):
        self.area = 0
        self.perimeter = 0
        self.variety = variety

    def price(self):
        return self.area * self.perimeter

    def __str__(self):
        value = f"{self.variety} => area: {self.area}, perimeter: {self.perimeter}, price: {self.price()}"
        return value


def get_perimeter(grid, x, y, value):
    counter = 0
    if x - 1 >= 0 and value in grid[y][x - 1]:
        counter += 1
    if y - 1 >= 0 and value in grid[y - 1][x]:
        counter += 1
    if x + 1 < len(grid[0]) and value in grid[y][x + 1]:
        counter += 1
    if y + 1 < len(grid) and value in grid[y + 1][x]:
        counter += 1
    return 4 - counter


def flood(grid, value, plot, x, y):
    if x < 0 or y < 0 or x >= len(grid[0]) or y >= len(grid):
        return
    if grid[y][x] != value:
        return
    plot.area += 1
    plot.perimeter += get_perimeter(grid, x, y, value)
    grid[y][x] = f"{value}0"
    flood(grid, value, plot, x + 1, y)
    flood(grid, value, plot, x - 1, y)
    flood(grid, value, plot, x, y + 1)
    flood(grid, value, plot, x, y - 1)


def part1(grid):
    map = []
    for y, row in enumerate(grid):
        for x, value in enumerate(row):
            if "0" not in value:
                plot = Plot(value)
                flood(grid, value, plot, x, y)
                map.append(plot)
    acc = 0
    dump(grid, map)
    for value in map:
        acc += value.price()

    return acc


def part2(grid):
    map = []
    for y, row in enumerate(grid):
        for x, value in enumerate(row):
            if "0" not in value:
                plot = Plot(value)
                flood(grid, value, plot, x, y)
                map.append(plot)
    acc = 0
    for value in map:
        acc += value.price()

    return acc


ans1 = part1(grid)
print(ans1)
assert ans1 == 1930

ans2 = part2(grid)
print(ans2)
assert ans2 == 1206
