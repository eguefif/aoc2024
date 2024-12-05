with open("../../inputs/d5", "r") as f:
    content = f.read()

parts = content.split("\n\n")
ordering = list(map(lambda x: x.split("|"), parts[0].split("\n")))
pages = list(map(lambda x: x.split(","), parts[1].split("\n")))


def check(a, b):
    for pair in ordering:
        if b == pair[0] and a == pair[1]:
            return False
    return True


def is_ordered(line):
    return all((check(a, b) for a, b in zip(line, line[1:])))


acc = 0
for line in pages[:-1]:
    if is_ordered(line) and len(line) > 0:
        n = len(line)
        idx = int(n / 2)
        acc += int(line[idx])

print(acc)
