with open("../inputs/d9") as f:
    content = f.read().strip()


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


def get_files_index(index):
    files = []
    idx = 0
    i = 0
    while i < len(index):
        file = {
            "size": index[i],
            "index": idx,
        }
        files.append(file)
        if i + 2 < len(index):
            idx += index[i] + index[i + 1]
            i += 2
        else:
            idx += index[i]
            i += 1
    return files


def look_for_space(file, spaces):
    for space in spaces:
        if space["index"] >= file["index"]:
            return -1
        if space["size"] >= file["size"]:
            retval = space["index"]
            return retval
    return -1


def copy2(idx, disk, file):
    for i in range(0, file["size"]):
        disk[idx + i] = disk[file["index"] + i]
        disk[file["index"] + i] = "."


def get_size(disk, i):
    counter = 0
    while i < len(disk) and disk[i] == ".":
        counter += 1
        i += 1
    return counter


def get_spaces_index(disk):
    spaces = []
    i = 0
    while i < len(disk):
        if disk[i] == ".":
            size = get_size(disk, i)
            space = {
                "size": size,
                "index": i,
            }
            spaces.append(space)
            i += size
        else:
            i += 1
    return spaces


def remap2(disk, index):
    files = get_files_index(index)
    spaces = get_spaces_index(disk)

    for file in files[::-1]:
        new_spot = look_for_space(file, spaces)
        if new_spot != -1:
            copy2(new_spot, disk, file)
            spaces = get_spaces_index(disk)
    return disk


def part2(line):
    disk = put_id(line)
    disk = remap2(disk, line)
    return calculate(disk)


ans = part1(line)
print(ans)
ans2 = part2(line)
print(ans2)
