defmodule Day6 do
  def parse(content) do
    content
    |> String.split("\n", trim: true)
    |> Enum.with_index()
    |> Enum.reduce(Map.new(), fn {line, y}, map ->
      line
      |> String.split("", trim: true)
      |> Enum.with_index()
      |> Enum.reduce(map, fn {datum, x}, map -> Map.put(map, {x, y}, datum) end)
    end)
  end

  def part1(map) do
    guard = Day6.init_guard(map)

    move_guard(guard, map, {0, -1})
    |> Map.to_list()
    |> Enum.filter(fn entry -> elem(entry, 1) == "X" end)
    |> Enum.count()
  end

  def move_guard({x, y}, map, {dx, dy}) do
    map = Map.replace(map, {x, y}, "X")
    next_position = {x + dx, y + dy}

    case Map.fetch(map, next_position) do
      {:ok, value} ->
        if value == "#" do
          d = change_dir({dx, dy})
          move_guard({x, y}, map, d)
        else
          move_guard(next_position, map, {dx, dy})
        end

      :error ->
        map
    end
  end

  def draw_map(map) do
    Enum.each(0..9, fn y ->
      Enum.each(0..9, fn x ->
        IO.write("#{Map.get(map, {x, y})}")
      end)

      IO.puts("")
    end)
  end

  def change_dir(dir) do
    case dir do
      {0, 1} -> {-1, 0}
      {-1, 0} -> {0, -1}
      {0, -1} -> {1, 0}
      {1, 0} -> {0, 1}
    end
  end

  def init_guard(data) do
    data
    |> Map.to_list()
    |> Enum.find(fn datum ->
      elem(datum, 1) == "^"
    end)
    |> elem(0)
  end

  def part2(map) do
    guard = Day6.init_guard(map)

    Day6.get_visited(guard, map, {0, -1}, MapSet.new())
    |> MapSet.to_list()
    |> Enum.filter(fn obstacle ->
      if guard != obstacle do
        map_to_check = Map.replace(map, obstacle, "#")
        history = MapSet.new()
        check_loop(map_to_check, guard, history, {0, -1})
      else
        false
      end
    end)
    |> Enum.count()
  end

  def get_visited({x, y}, map, {dx, dy}, visited) do
    next_position = {x + dx, y + dy}

    case Map.fetch(map, next_position) do
      {:ok, value} ->
        if value == "#" do
          d = change_dir({dx, dy})
          get_visited({x, y}, map, d, visited)
        else
          visited = MapSet.put(visited, next_position)
          get_visited(next_position, map, {dx, dy}, visited)
        end

      :error ->
        visited
    end
  end

  def check_loop(map, {x, y}, history, {dx, dy}) do
    history = MapSet.put(history, {{x, y}, {dx, dy}})
    next = {x + dx, y + dy}
    case Map.fetch(map, next) do
      {:ok, value} ->
          if MapSet.member?(history, {next, {dx, dy}}) do
            true 
          else 
            if value == "#" do
              dir = change_dir({dx, dy})
              check_loop(map, {x, y}, history, dir)
            else
              check_loop(map, next, history, {dx, dy})
            end
          end
      :error -> false
    end
  end
end

input = {"../input/day6.txt", 4776, 1586}
#input = {"../input/day6-example.txt", 41, 6}

data =
  elem(input, 0)
  |> File.read!()
  |> Day6.parse()

p1 = Day6.part1(data)
IO.puts("Part1: #{p1} correct?  #{p1 == elem(input, 1)}")

p2 = Day6.part2(data)
IO.puts("Part2: #{p2} correct?  #{p2 == elem(input, 2)}")
