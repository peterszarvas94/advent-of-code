input =
  case File.read("day-2.input") do
    {:ok, content} ->
      content
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line
        |> String.split()
        |> Enum.map(&String.to_integer/1)
      end)

    {:error, reason} ->
      IO.puts("Error: #{reason}")
      []
  end

# IO.inspect(input, label: "Input", charlists: :as_lists)

defmodule AOC do
  def safe(input) do
    desc =
      input
      |> Enum.filter(fn line ->
        case line do
          [first, second | _tail] -> first > second
          _ -> false
        end
      end)
      |> Enum.filter(fn line ->
        line
        |> Enum.reduce({true, nil}, fn
          current, {true, nil} ->
            {true, current}

          current, {true, prev} when 1 <= prev - current and prev - current <= 3 ->
            {true, current}

          _, {_, _} ->
            {false, nil}
        end)
        |> elem(0)
      end)

    asc =
      input
      |> Enum.filter(fn line ->
        case line do
          [first, second | _tail] -> second > first
          _ -> false
        end
      end)
      |> Enum.filter(fn line ->
        line
        |> Enum.reduce({true, nil}, fn
          current, {true, nil} ->
            {true, current}

          current, {true, prev} when 1 <= current - prev and current - prev <= 3 ->
            {true, current}

          _, {_, _} ->
            {false, nil}
        end)
        |> elem(0)
      end)

    Enum.concat(desc, asc)
    |> Enum.count()
  end
end

# part 1

part1 = AOC.safe(input)

IO.inspect(part1, label: "part 1", charlists: :as_lists)

# part 2

part2 =
  input
  |> Enum.map(fn line ->
    line
    |> Enum.with_index()
    |> Enum.map(fn
      {_, index} -> List.delete_at(line, index)
    end)
    |> AOC.safe()
  end)
  |> Enum.count(fn x -> x > 0 end)

IO.inspect(part2, label: "part 2", charlists: :as_lists)
