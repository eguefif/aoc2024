WIDTH = 101
HEIGHT = 103

# WIDTH = 11
# HEIGHT = 7


class Robot:
    def __init__(self, line):
        splits = line.split()
        x, y = splits[0][2:].split(",")
        dx, dy = splits[1][2:].split(",")
        self.x, self.y = int(x), int(y)
        self.dx, self.dy = int(dx), int(dy)

    def __repr__(self):
        return f"Position: {self.x}, {self.y}, Velocity: {self.dx}, {self.dy}"

    def move(self):
        self.x = (self.x + self.dx) % WIDTH
        self.y = (self.y + self.dy) % HEIGHT


with open("../input/day14.txt", "r") as f:
    content = f.readlines()
    robots = [Robot(line) for line in content]


def print_map(map):
    for y in range(HEIGHT):
        for x in range(WIDTH):
            if map[y][x] == 0:
                print(".", end="")
            else:
                print(map[y][x], end="")
        print("")


def build_map(rbts):
    map = []
    for y in range(HEIGHT):
        line = [0 for _ in range(WIDTH)]
        for x in range(WIDTH):
            for robot in rbts:
                if robot.x == x and robot.y == y:
                    line[x] += 1
        map.append(line)
    return map


def get_quadrant(x, y):
    if x < WIDTH / 2:
        if y < HEIGHT / 2:
            return 0
        else:
            return 2
    else:
        if y < HEIGHT / 2:
            return 1
        else:
            return 3


def get_safe_zone(map):
    q = [0, 0, 0, 0]
    for y in range(HEIGHT):
        if y == int(HEIGHT / 2):
            continue
        for x in range(WIDTH):
            if int(WIDTH / 2) == x or map[y][x] == ".":
                continue
            quadrant = get_quadrant(x, y)
            q[quadrant] += map[y][x]

    return q[0] * q[1] * q[2] * q[3]


def get_ans1(rbts):
    for _ in range(100):
        for rbt in rbts:
            rbt.move()
    map = build_map(rbts)
    return get_safe_zone(map)


ans1 = get_ans1(robots.copy())
print(ans1)
