content =
  File.read!("day-4.input")
  |> String.split("\n", trim: true)

# IO.inspect(content, label: "Input", charlists: :as_lists)

defmodule AOC do
  def get_coords(input) do
    matrix =
      input
      |> Enum.map(fn item ->
        String.graphemes(item)
      end)

    items_with_y =
      Enum.with_index(matrix)

    items_with_yx =
      for {row, y} <- items_with_y do
        for {item, x} <- Enum.with_index(row) do
          {item, x, y}
        end
      end
      |> List.flatten()

    items_with_yx
  end

  def collect_by_grouping_rule(coords, group_fn) do
    coords
    |> Enum.group_by(group_fn)
    |> Enum.map(fn {_key, data} -> data end)
  end

  def find_all_str_in_matrix(matrix, str) do
    matrix
    |> Enum.flat_map(fn list ->
      [list, Enum.reverse(list)]
      |> Enum.map(&find_all_str_in_direction(&1, str))
      |> Enum.filter(fn
        [_a] -> true
        _ -> false
      end)
    end)
  end

  def find_all_str_in_direction(list, str) do
    input = Enum.map(list, fn {letter, _x, _y} -> letter end) |> Enum.join("")
    {_item, _x, y} = Enum.at(list, 0)
    do_find_all_str(input, str, 0, y, [])
  end

  defp do_find_all_str(input, str, start, y, acc) do
    if start >= String.length(input) do
      Enum.reverse(acc)
    else
      sub = String.slice(input, start..-1//1) |> String.slice(0, String.length(str))

      if String.contains?(sub, str) do
        do_find_all_str(input, str, start + 1, y, [{start, y} | acc])
      else
        do_find_all_str(input, str, start + 1, y, acc)
      end
    end
  end
end

coords = AOC.get_coords(content)

vertical = AOC.collect_by_grouping_rule(coords, fn {_item, _x, y} -> y end)
horizontal = AOC.collect_by_grouping_rule(coords, fn {_item, x, _y} -> x end)
diag1 = AOC.collect_by_grouping_rule(coords, fn {_item, x, y} -> x - y end)
diag2 = AOC.collect_by_grouping_rule(coords, fn {_item, x, y} -> x + y end)

vertical_coords = AOC.find_all_str_in_matrix(vertical, "XMAS")
horizontal_coords = AOC.find_all_str_in_matrix(horizontal, "XMAS")
diag1_coords = AOC.find_all_str_in_matrix(diag1, "XMAS")
diag2_coords = AOC.find_all_str_in_matrix(diag2, "XMAS")

# IO.inspect(vertical_coords, label: "vertical_coords", charlists: :as_lists)
# IO.inspect(horizontal_coords, label: "horizontal_coords", charlists: :as_lists)
# IO.inspect(diag1_coords, label: "diag1_coords", charlists: :as_lists)
# IO.inspect(diag2_coords, label: "diag2_coords", charlists: :as_lists)

part1 =
  vertical_coords
  |> Enum.concat(horizontal_coords)
  |> Enum.concat(diag1_coords)
  |> Enum.concat(diag2_coords)
  |> Enum.count()

IO.inspect(part1, label: "part1", charlists: :as_lists)

# works for sample, not for input
