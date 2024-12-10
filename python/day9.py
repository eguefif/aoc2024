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


def copy(disk, fr, to, n):
    for i in range(0, n):
        disk[to + i] = disk[fr - i]
        disk[fr - i] = "."


class Cursor:
    def __init__(self, fr, to):
        self.fr = fr
        self.to = to


def is_disk_finishing_with_a_file(index):
    return len(index) % 2 != 0


def dump(disk, index, disk_c, index_c):
    print("".join([str(v) for v in disk]))
    print(
        " " * (disk_c.to - 1),
        "|",
        " " * (disk_c.fr - disk_c.to - 3),
        "|",
        " " * 5,
        disk_c.to,
        " ",
        disk_c.fr,
    )
    print("".join([str(v) for v in index]))
    print(
        " " * (index_c.to - 1),
        "|",
        " " * (index_c.fr - index_c.to - 3),
        "|",
        " " * 5,
        index_c.to,
        " ",
        index_c.fr,
    )

    print()


def remap2(disk, index):
    if is_disk_finishing_with_a_file(index):
        disk_c = Cursor(fr=len(disk) - 1, to=index[0])
        index_c = Cursor(fr=len(index) - 1, to=1)
    else:
        disk_c = Cursor(fr=len(disk) - 1 - index[-1], to=index[0])
        index_c = Cursor(fr=len(index) - 1, to=1)

    while index_c.to < len(index) and disk_c.to < len(disk):
        while index_c.fr > index_c.to and disk_c.fr > 0:
            dump(disk, index, disk_c, index_c)
            if index[index_c.to] >= index[index_c.fr]:
                n = index[index_c.fr]
                copy(disk, disk_c.fr, disk_c.to, n)
                index[index_c.to] -= n
                disk_c.to += n

                index_c.fr -= 1
                index[index_c.fr] += n
                disk_c.fr -= index[index_c.fr]
                index_c.fr -= 1
                index.pop()
            else:
                n = index[index_c.fr] + index[index_c.fr - 1]
                index_c.fr -= 2
                disk_c.fr -= n
        if is_disk_finishing_with_a_file(index):
            disk_c.fr = len(disk) - 1
            index_c.fr = len(disk) - 1
        else:
            disk_c.fr = len(disk) - 1 - index[-1]
            index_c.fr = len(disk) - 1 - index[-1]
        n = index[index_c.to] + index[index_c.to + 1]
        index_c.to += 2
        disk_c.to += n

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
