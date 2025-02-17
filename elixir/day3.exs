defmodule Day3 do
  def parse(content) do
    String.trim(content)
  end

  def part1(input) do
    Regex.scan(~r/mul\((\d+),(\d+)\)/, input)
      |> Enum.map(fn [_, x, y] -> {String.to_integer(x), String.to_integer(y)} end)
      |> Enum.reduce(0, fn {x, y}, acc -> acc + x * y end)
  end

  def part2(input) do
    Regex.scan(~r/(?>mul\((\d+),(\d+)\))|(?>do\(\))||(?>don't\(\))/, input)
      |>Enum.filter(&(&1 != [""]))
      |>Enum.reduce({true, 0}, fn token, {continue?, total} ->
        case {continue?, token} do
          {_, ["do()"]} -> {true, total}
          {_, ["don't()"]} -> {false, total}
          {true, token} -> {true, total + multiply(token)}
          {false, _} -> {false, total}
        end
      end)
      |>elem(1)
  end

  def multiply([_ | op]) do
    String.to_integer(Enum.at(op, 0)) * String.to_integer(Enum.at(op, 1))

  end
end

data = "../input/day3.txt"
  |> File.read!()
  |> Day3.parse()

IO.puts "Part1: #{Day3.part1(data)}"
IO.puts "Part2: #{Day3.part2(data)}"
