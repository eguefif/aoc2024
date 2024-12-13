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

    def __str__(self):
        retval = f"A({self.a[0]}, {self.a[1]}), B({self.b[0], self.b[1]})"
        retval += f" Prize: ({self.prize[0], self.prize[1]})"
        return retval


with open("../inputs/exemple", "r") as f:
    content = f.read()
    machines = content.split("\n\n")
    machines = [Machine(machine.strip()) for machine in machines]
[print(machine) for machine in machines]
