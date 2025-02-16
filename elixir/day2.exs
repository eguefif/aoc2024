defmodule Day2 do
  def parse(input) do
    input
      |> Enum.map(fn line -> 
        line |> String.split(" ", trim: true) |> Enum.map(&String.to_integer/1) 
    end)
  end

  def part1(data) do
    data
      |> Enum.count(&safe?/1)
  end

  defp safe?([a, a | _]), do: false
  defp safe?([a, b | rest]) do
    sign = if b - a > 0, do: 1, else: -1
    check_gap?([a, b | rest], sign)
  end

  defp check_gap?([_], _), do: true
  defp check_gap?([a, b | rest], sign) do
    ((b - a) * sign in 1..3) && check_gap?([b | rest], sign)
  end

  def part2(input) do
    input
      |>Enum.count(fn x -> check_damp(x, []) end)
  end

  defp check_damp([], prefix) do
    safe?(prefix)
  end

  defp check_damp([head | tail], prefix) do
    safe?(Enum.reverse(tail, prefix)) or check_damp(tail, [head | prefix])
  end
end

data = "../input/day2.txt"
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Day2.parse()

IO.puts "Part1: #{Day2.part1(data)}"
IO.puts "Part2: #{Day2.part2(data)}"
