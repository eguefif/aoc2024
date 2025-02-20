defmodule Day5 do
  def parse(content) do
    [orders_data, updates_data] = content
    |>String.split("\n\n", trim: true)
    orders = orders_data
      |>String.split("\n", trim: true)
      |>Enum.map(&(String.split(&1, "|")))
      |>Enum.map(&(List.to_tuple(&1)))
      |>Enum.map(fn {n1, n2} -> {String.to_integer(n1), String.to_integer(n2)} end)
    updates = updates_data
      |>String.split("\n", trim: true)
      |>Enum.map(fn line ->
        String.split(line, ",")
          |>Enum.map(fn n -> String.to_integer(n) end) end)
    {orders, updates}
  end

  def part1({orders, updates}) do
    updates
    |>Enum.filter(&(ordered?(&1, orders)))
    |>Enum.map(fn update ->
      mid = round(length(update) / 2) - 1
      Enum.at(update, mid)
    end)
    |>Enum.sum()
  end

  def ordered?(update, orders) do
    update
    |>Enum.chunk_every(2, 1)
    |>Enum.map(fn chunk ->
      length(chunk) == 1 || not ({Enum.at(chunk, 1), Enum.at(chunk, 0)} in orders)
    end)
    |>Enum.all?()
  end

  def part2({orders, updates}) do
    updates
    |>Enum.filter(&(not ordered?(&1, orders)))
    |>Enum.map(&(Enum.sort(&1, fn a, b -> comp({a, b}, orders) end)))
    |>Enum.map(fn update ->
      mid = round(length(update) / 2) - 1
      Enum.at(update, mid)
    end)
    |>Enum.sum()
  end

  def comp({a, b}, orders) do
    not ({b, a} in orders)
  end
end

input = {"../input/day5.txt", 6505, 6897}
#input = {"../input/day5-example.txt", 143, 123}

data = elem(input, 0)
  |>File.read!()
  |>Day5.parse()

p1 = Day5.part1(data)
IO.puts "Part1: #{p1} correct? #{p1 == elem(input, 1)}"

p2 = Day5.part2(data)
IO.puts "Part2: #{p2} correct? #{p2 == elem(input, 2)}"

