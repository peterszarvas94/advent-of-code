file = File.read("day-1.input")

{first_column, second_column} =
  case file do
    {:ok, body} ->
      body
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        [num1, num2] = String.split(line)
        {String.to_integer(num1), String.to_integer(num2)}
      end)
      |> Enum.unzip()

    {:error, reason} ->
      IO.puts("Error: #{reason}")
      {[], []}
  end

# part 1

sorted_first = Enum.sort(first_column)
sorted_second = Enum.sort(second_column)

differences =
  Enum.zip(sorted_first, sorted_second)
  |> Enum.map(fn {num1, num2} -> abs(num1 - num2) end)
  |> Enum.sum()

IO.puts("part 1: #{differences}")

# part 2

similarity =
  Enum.map(
    first_column,
    fn a ->
      Enum.count(
        second_column,
        fn b -> a == b end
      ) * a
    end
  )
  |> Enum.sum()

# counts = Enum.frequencies(second_column)
#
# similarity =
#   Enum.reduce(first_column, 0, fn a, acc ->
#     acc + Map.get(counts, a, 0) * a
#   end)

IO.puts("part 2: #{similarity}")
