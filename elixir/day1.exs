content = File.read!("../input/day1.txt")
lines = String.split(content, "\n")

l1 = for line <- lines, do: Enum.at(String.split(line), 0)
l2 = for line <- lines, do: Enum.at(String.split(line), 1)

l1 = Enum.filter(l1, &(&1 != nil))
l2 = Enum.filter(l2, &(&1 != nil))

l1 = Enum.sort(l1)
l2 = Enum.sort(l2)

l = for i <- 0..(length(l1) - 1), do: String.to_integer(Enum.at(l1, i)) - String.to_integer(Enum.at(l2, i))

l = for i <- l, do: abs(i)
IO.puts Enum.reduce(l, 0, fn item, acc -> item + acc end)
