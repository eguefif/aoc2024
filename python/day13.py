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

    def get_mina(self):
        max1 = self.prize[0] / self.a[0]
        max2 = self.prize[1] / self.a[1]
        return max(int(max1), int(max2))

    def __str__(self):
        retval = f"A({self.a[0]}, {self.a[1]}), B({self.b[0], self.b[1]})"
        retval += f" Prize: ({self.prize[0], self.prize[1]})"
        return retval

    # with open("../inputs/exemple", "r") as f:


with open("../inputs/d13", "r") as f:
    content = f.read()
    machines = content.split("\n\n")
    machines = [Machine(machine.strip()) for machine in machines]


def get_cost(machine):
    mina = machine.get_mina()
    prices = []

    for a in range(0, mina):
        new_x = machine.prize[0] - machine.a[0] * a
        new_y = machine.prize[1] - machine.a[1] * a
        if new_x % machine.b[0] == 0 and new_y % machine.b[1] == 0:
            b = int(new_x / machine.b[0])
            prices.append(a * 3 + b)

    if len(prices) == 0:
        return 0
    return min(prices)


def part1(machines):
    acc = 0
    for machine in machines:
        retval = get_cost(machine)
        acc += retval
    return acc


ans1 = part1(machines)

print(ans1)

assert ans1 == 480
# 5466
