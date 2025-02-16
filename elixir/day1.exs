defmodule Day1 do
  def parse_input(input) do
    input
      |> Stream.map(fn line -> line |> String.split() |> Enum.map(&String.to_integer/1) end)
      |> Stream.map(&List.to_tuple/1)
      |> Enum.unzip()
  end

  def part1(l1, l2) do
    Enum.zip(Enum.sort(l1), Enum.sort(l2))
      |> Enum.map(fn {a, b} -> abs(a - b) end)
      |> Enum.sum()
  end

  def part2(l1, l2) do
    frequencies = Enum.frequencies(l2)

    l1 
      |> Enum.map(fn x -> x * Map.get(frequencies, x, 0) end)
      |> Enum.sum()
  end
end

{l1, l2} = "../input/day1.txt" 
  |> File.stream!()
  |> Day1.parse_input()

IO.puts "Part1: #{Day1.part1(l1, l2)}"
IO.puts "Part2: #{Day1.part2(l1, l2)}"
