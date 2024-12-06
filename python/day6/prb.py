with open("../../inputs/d6", "r") as f:
    content = f.readlines()

ans = 0
asn2 = 0


def get_input(content):
    input = [list(line.strip()) for line in content]
    return input


def is_moving_out(guard, input, direction):
    x = guard[0] + direction[0]
    y = guard[1] + direction[1]
    if x < 0 or x >= len(input[0]) or y < 0 or y >= len(input):
        print(x, y)
        return True

    return False


def next_move_possible(guard, input, direction):
    x = guard[0] + direction[0]
    y = guard[1] + direction[1]
    if input[y][x] == "#":
        return False

    input[y][x] = "x"
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


def count_positions(input):
    acc = 0
    for row in input:
        acc += row.count("x")
    return acc


def part1(input):
    direction = [0, -1]
    guard = get_guard(input)

    while not is_moving_out(guard, input, direction):
        if next_move_possible(guard, input, direction):
            guard = move(guard, direction)
        else:
            direction = turn(direction)

    for row in input:
        print("".join(row))
    return count_positions(input)


def get_guard(input):
    for y in range(0, len(input)):
        for x in range(0, len(input[0])):
            if input[y][x] == "^":
                return [x, y]


def part2(input):
    direction = [0, -1]
    guard = get_guard(input)
    first_hit = []
    last_hit = []

    while not is_moving_out(guard, input, direction):
        if next_move_possible(guard, input, direction):
            guard = move(guard, direction)
        else:
            direction = turn(direction)

    for row in input:
        print("".join(row))
    return count_positions(input)


input = get_input(content)

ans = part1(input)
ans2 = part2(input)

print("Part1: ", ans)
print("Part2: ", ans2)

assert ans == 41
assert ans2 == 6
