class Machine:
    def __init__(self, machine):
        a, b, prize = self.parse(machine)
        self.a = a
        self.b = b
        self.prize = prize

    def parse(self, machine):
        splits = [m.strip() for m in machine.split("\n")]
        a = self.get_x_y(splits[0])
        b = self.get_x_y(splits[1])
        prize = self.get_x_y_prize(splits[2])
        return a, b, prize

    def get_x_y(self, line):
        splits = line.split(" ")
        x = splits[2].split("+")[1].replace(",", "")
        y = splits[3].split("+")[1].replace(",", "")
        return (int(x), int(y))

    def get_x_y_prize(self, line):
        splits = line.split(" ")
        x = splits[1].split("=")[1].replace(",", "")
        y = splits[2].split("=")[1].replace(",", "")
        return (int(x), int(y))

    def solve(self):
        d = self.a[0] * self.b[1] - self.a[1] * self.b[0]
        a = self.prize[0] * self.b[1] - self.prize[1] * self.b[0]
        b = self.prize[1] * self.a[0] - self.prize[0] * self.a[1]
        if a % d == 0 and b % d == 0:
            return 3 * (a / d) + b / d
        return 0

    def rescale(self):
        self.prize = self.prize[0] + 10000000000000, self.prize[1] + 10000000000000

    def __str__(self):
        retval = f"A({self.a[0]}, {self.a[1]}), B({self.b[0], self.b[1]})"
        retval += f" Prize: ({self.prize[0], self.prize[1]})"
        return retval


with open("../inputs/d13", "r") as f:
    content = f.read()
    machines = content.split("\n\n")
    machines = [Machine(machine.strip()) for machine in machines]


def dump(machines):
    [print(machine) for machine in machines]


def part1(machines):
    acc = 0
    for machine in machines:
        acc += machine.solve()
    return acc


def part2(machines):
    acc = 0
    for machine in machines:
        machine.rescale()
        acc += machine.solve()
    return acc


ans1 = part1(machines)
ans2 = part2(machines)

print(ans1)
print(ans2)
