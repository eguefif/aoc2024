with open("../inputs/exemple", "r") as f:
    content = f.readlines()
    grid = [list(line.strip()) for line in content]


def dump(grid, map):
    [print("".join(line)) for line in grid]

    for key, value in map.items():
        print(f"Key {key}: {value}")


class Plot:
    def __init__(self, variety):
        self.area = 0
        self.perimeter = 0
        self.variety = None

    def price(self):
        return self.area * self.perimeter

    def __str__(self):
        value = f"{self.variety} => area: {self.area}, perimeter: {self.perimeter}, price: {self.price()}"
        return value


def get_perimeter(grid, x, y, value):
    counter = 0
    if x - 1 >= 0 and grid[y][x - 1] == value:
        counter += 1
    if y - 1 >= 0 and grid[y - 1][x] == value:
        counter += 1
    if x + 1 < len(grid[0]) and grid[y][x + 1] == value:
        counter += 1
    if y + 1 < len(grid) and grid[y + 1][x] == value:
        counter += 1
    return 4 - counter


def part1(grid):
    map = {}
    for y, row in enumerate(grid):
        for x, value in enumerate(row):
            if len(map.keys()) == 0 or value not in map.keys():
                new_plot = Plot(value)
                map[value] = new_plot
            map[value].area += 1
            map[value].perimeter += get_perimeter(grid, x, y, value)
    acc = 0
    dump(grid, map)
    for value in map.values():
        acc += value.price()

    return acc


ans1 = part1(grid)
print(ans1)
assert ans1 == 1930
