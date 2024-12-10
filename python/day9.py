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


def init_cursor(disk, index):
    disk_c = Cursor(0, 0)

    i = 0
    while i < len(disk):
        if disk[i] != ".":
            i += 1
            continue
        disk_c.to = i
        break

    i = len(disk) - 1
    while i >= 0:
        if disk[i] == ".":
            i -= 1
            continue
        disk_c.fr = i
        break

    index_c = Cursor(0, len(index) - 1)

    i = len(index) - 1
    if disk[len(disk) - 1] != ".":
        index_c.fr = i
    else:
        while i > 0:
            if i % 2 != 0:
                i -= 1
                continue
            if index[i] != 0:
                index_c.fr = i
                break

            i -= 1

    for i, v in enumerate(index):
        if i % 2 == 0:
            i += 1
            continue
        if v != 0:
            index_c.to = i
            break
        i += 1

        if disk_c.to > disk_c.fr:
            print("t", disk_c.to)
            print(disk_c.fr)
            disk_c.to = -1
    return disk_c, index_c


def init(disk):
    index = []
    counter = 0
    for i, c in enumerate(disk):
        if (
            i + 1 < len(disk)
            and c != disk[i + 1]
            and type(c) is type(disk[i + 1])
            and i != 0
        ):
            index.append(counter)
            index.append(0)
            counter = 0
        elif i + 1 < len(disk) and c is not disk[i + 1]:
            index.append(counter)
            counter = 0
        elif i + 1 >= len(disk):
            index.append(counter)
            break
        counter += 1

    disk_c, index_c = init_cursor(disk, index)

    return index, disk_c, index_c


def remap2(disk):
    print(len(disk))
    index, disk_c, index_c = init(disk)
    print(len(index))
    counter = 0

    while disk_c.to != -1 and counter < 10:
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
            counter += 1

        index, disk_c, index_c = init(disk)

    return disk


def part2(line):
    disk = put_id(line)
    disk = remap2(disk)
    return calculate(disk)


ans = part1(line)
ans2 = part2(line)
print(ans)
print(ans2)
assert ans == 1928
assert ans2 == 2858