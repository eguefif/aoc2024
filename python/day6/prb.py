import collections

with open("../../inputs/exemple", "r") as f:
    content = f.readlines()

ans = 0
asn2 = 0


def get_inp(content):
    inp = [list(line.strip()) for line in content]
    return inp


def is_moving_out(guard, inp, direction):
    x = guard[0] + direction[0]
    y = guard[1] + direction[1]
    if x < 0 or x >= len(inp[0]) or y < 0 or y >= len(inp):
        return True

    return False


def next_move_possible(guard, inp, direction, check=True):
    x = guard[0] + direction[0]
    y = guard[1] + direction[1]
    if inp[y][x] == "#":
        return False

    if check:
        inp[y][x] = "x"
    return True


def move(guard, direction):
    guard[0] = guard[0] + direction[0]
    guard[1] = guard[1] + direction[1]
    return guard


def turn(direction):
    match direction:
        case (0, -1):
            return (1, 0)
        case (1, 0):
            return (0, 1)
        case (0, 1):
            return (-1, 0)
        case (-1, 0):
            return (0, -1)


def count_positions(inp):
    acc = 0
    for row in inp:
        acc += row.count("x")
    return acc


def print_inp(inp):
    for row in inp:
        print("".join(row))


def part1(inp):
    direction = [0, -1]
    guard = get_guard(inp)

    while not is_moving_out(guard, inp, direction):
        if next_move_possible(guard, inp, direction):
            guard = move(guard, direction)
        else:
            direction = turn(direction)

    print_inp(inp)
    return count_positions(inp)


def get_guard(inp):
    for y in range(0, len(inp)):
        for x in range(0, len(inp[0])):
            if inp[y][x] == "^":
                return [x, y]


def count_objects(inp):
    acc = 0
    for row in inp:
        acc += row.count("O")
    return acc


def is_collision(guard, new_direction, inp):
    if new_direction[1] != 0:
        for y in range(guard[1], len(inp)):
            if inp[y][guard[0]] == "#":
                return guard[0], y
    else:
        for x in range(guard[1], len(inp[0])):
            if inp[guard[1]][x] == "#":
                return x, guard[1]
    return -1, -1


def check_square(x_b, y_b, direction, guard_b, inp):
    count = 0
    print("direction: ", direction)
    x = x_b - direction[0]
    y = y_b - direction[1]
    guard = [x, y]
    direction = turn(direction)
    while not is_moving_out(guard, inp, direction):
        if count == 2:
            if guard == guard_b:
                return True
            elif next_move_possible(guard, inp, direction, False):
                guard = move(guard, direction)
            else:
                return False
        elif next_move_possible(guard, inp, direction, False):
            guard = move(guard, direction)
        else:
            count += 1
            direction = turn(direction)
    return False


def check_trap(guard, direction, inp):
    trap = (guard[0] + direction[0], guard[1] + direction[1])
    try:
        if inp[trap[1]][trap[0]] == "#":
            return
    except Exception:
        return
    new_direction = turn(direction)
    x, y = is_collision(guard, new_direction, inp)
    if x == -1:
        return
    print(x, y)
    if check_square(x, y, new_direction, guard, inp):
        inp[trap[0]][trap[1]]


def part2(inp):
    direction = [0, -1]
    guard = get_guard(inp)

    while not is_moving_out(guard, inp, direction):
        if next_move_possible(guard, inp, direction, False):
            guard = move(guard, direction)
        else:
            direction = turn(direction)
            check_trap(guard, direction, inp)

    print_inp(inp)
    return count_objects(inp)


inp = get_inp(content)
inp2 = get_inp(content)

ans = part1(inp)
ans2 = part2(inp2)

print("Part1: ", ans)
print("Part2: ", ans2)

assert ans == 41
assert ans2 == 6
