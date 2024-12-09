with open("../inputs/d9") as f:
    content = f.read().strip()


content = "2333133121414131402"

line = [int(a) for a in list(content)]


def put_id(line):
    counter = 0
    retval = []
    for i, c in enumerate(line):
        if i % 2 == 0:
            for _ in range(0, c):
                retval.append(counter)
            counter += 1
        else:
            for _ in range(0, c):
                retval.append(".")
    return retval


def remap(disk):
    from_idx = len(disk) - 1
    to = 0
    while to < from_idx:
        while disk[to] != "." and to < from_idx:
            to += 1
        while disk[from_idx] == "." and from_idx > to:
            from_idx -= 1
        tmp = disk[to]
        disk[to] = disk[from_idx]
        disk[from_idx] = tmp
        to += 1
        from_idx -= 1
    return disk


def calculate(disk):
    counter = 0
    for i, v in enumerate(disk):
        if v == ".":
            continue
        counter += i * v
    return counter


def part1(line):
    disk = put_id(line)
    disk = remap(disk)
    return calculate(disk)


def copy(disk, to, fr, n):
    for i in range(0, n):
        tmp = disk[fr - i]
        disk[fr - i] = disk[to + i]
        disk[to + i] = tmp


def remap2(disk, index):
    from_d = len(disk) - 1
    to_d = 0

    spaces = [value for i, value in index if i % 2 != 0]
    num_sizes = [value for i, value in index if i % 2 == 0]
    nsi = len(num_sizes) - 1
    spi = 0

    while disk[to_d] != "." and to_d < len(disk):
        to_d += 1
    while disk[from_d] == "." and from_d >= 0:
        from_d -= 1

    while to_d < len(disk):
        if index[to_i] >= index[from_i]:
            copy(disk, to_d, from_d, length)
            number_size = index[from_id]
            to_d += length
            from_d -= length

    return disk


def part2(line):
    disk = put_id(line)
    disk = remap2(disk, line)
    return calculate(disk)


ans = part1(line)
ans2 = part2(line)
print(ans)
print(ans2)
assert ans == 1928
assert ans2 == 2858
