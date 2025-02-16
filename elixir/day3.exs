defmodule Day3 do
  def parse(content) do
    String.trim(content)
  end

  def part1(input) do
    Regex.scan(~r/mul\((\d+),(\d+)\)/, input)
      |> Enum.map(fn [_, x, y] -> {String.to_integer(x), String.to_integer(y)} end)
      |> Enum.reduce(0, fn {x, y}, acc -> acc + x * y end)
  end
end

data = "../input/day3.txt"
  |> File.read!()
  |> Day3.parse()

IO.puts "Part1: #{Day3.part1(data)}"
