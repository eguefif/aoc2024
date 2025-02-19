defmodule Day4 do
  def parse(input) do
    input
      |>Enum.with_index()
      |>Enum.reduce(Map.new(), &parse_row/2)
  end

  def parse_row({row, y}, map) do
    row
      |>String.graphemes
      |>Enum.with_index()
      |>Enum.reduce(map, fn {letter, x}, map ->
        Map.put(map, {x, y}, letter)
      end)
  end

  def part1(grid) do
    grid
      |>find_coords("X")
      |>Enum.flat_map(&find_xmas(grid, &1))
      |>length()
  end

  def find_coords(grid, check) do
    grid
    |>Map.filter(fn {_coord, letter} -> letter == check end)
  end

  def find_xmas(grid, {key, _}) do
    [{1, 1}, {1, 0}, {1, -1}, {0, -1}, {-1, -1}, {-1, 0}, {-1, 1}, {0, 1}] 
    |>Enum.filter(fn dir ->
      matches?(key, dir, 1, "M", grid) &&
      matches?(key, dir, 2, "A", grid) &&
      matches?(key, dir, 3, "S", grid)
    end)
  end

  def matches?({start_x, start_y}, {dx, dy}, coef, letter, grid) do
    letter == Map.get(grid, {start_x + dx * coef, start_y + dy * coef})
  end
end

grid = "../input/day4.txt"
  |>File.read!()
  |>String.split("\n", trim: true)
  |>Day4.parse()

IO.puts "Part1: #{Day4.part1(grid)}"
IO.puts "Part2: #{Day4.part2(grid)}"
