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


def is_stuck(guard, direction, inp):
    saved_guard = guard.copy()
    saved_direction = direction

    while not is_moving_out(guard, inp, direction):
        if next_move_possible(guard, inp, direction, False):
            guard = move(guard, direction)
        else:
            direction = turn(direction)
        if saved_guard == guard and saved_direction == direction:
            return True
    return False


def check_trap(guard, direction, inp):
    trap = (guard[0] + direction[0], guard[1] + direction[1])
    try:
        if inp[trap[1]][trap[0]] == "#":
            return
    except Exception:
        return
    save = inp[trap[1]][trap[0]]
    if not is_stuck(guard.copy(), turn(direction), inp):
        inp[trap[1]][trap[0]] = save
    else:
        inp[trap[1]][trap[0]] = "O"


def part2(inp):
    direction = [0, -1]
    guard = get_guard(inp)

    while not is_moving_out(guard, inp, direction):
        check_trap(guard, direction, inp)
        if next_move_possible(guard, inp, direction, False):
            guard = move(guard, direction)
        else:
            direction = turn(direction)

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
