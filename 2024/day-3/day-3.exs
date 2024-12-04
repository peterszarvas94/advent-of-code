content =
  File.read!("day-3.input")
  |> String.split("\n", trim: true)
  |> Enum.join("")

IO.inspect(content, label: "Input", charlists: :as_lists)

defmodule AOC do
  def get_numbers(input) do
    input
    |> Enum.at(0)
    # map into [a, b]
    |> String.split(",")
  end

  def parse_integer(nil), do: nil

  def parse_integer(value) do
    case Integer.parse(value) do
      {int, _rest} -> int
      :error -> nil
    end
  end

  def doOrDont(input) do
    do_ = String.contains?(input, "do()")
    dont = String.contains?(input, "don't()")

    cond do
      do_ -> true
      dont -> false
      true -> nil
    end
  end
end

# part 1

part1 =
  content
  # get the text between mul( and )
  |> String.split("mul(")
  |> Enum.map(fn item -> String.split(")") end)
  |> Enum.map(&AOC.get_numbers/1)
  # filter out non-2-length items
  |> Enum.filter(fn
    item when is_list(item) and length(item) == 2 -> true
    _ -> false
  end)
  # try to parse it
  |> Enum.map(fn
    [a, b] ->
      case {Integer.parse(a), Integer.parse(b)} do
        {{anum, ""}, {bnum, ""}} when 0 < anum and anum < 1000 and 0 < bnum and bnum < 1000 ->
          [anum, bnum]

        _ ->
          nil
      end

    _ ->
      nil
  end)
  # filter out if could not be parsed
  |> Enum.filter(fn
    nil -> false
    _ -> true
  end)
  # get result
  |> Enum.reduce(0, fn [a, b], acc -> acc + a * b end)

IO.inspect(part1, label: "Part 1", charlists: :as_lists)

# part 2

part2 =
  content
  |> String.split("mul(")
  |> IO.inspect(label: "split1", charlists: :as_lists)
  |> Enum.map(fn item -> String.split(item, ",", parts: 2) end)
  |> IO.inspect(label: "split2", charlists: :as_lists)
  |> Enum.map(fn [item | tail] -> [AOC.parse_integer(item) | tail] end)
  |> IO.inspect(label: "parse1", charlists: :as_lists)
  |> Enum.map(fn
    [a] -> [a, nil, nil]
    [a, b] -> Enum.concat([a], String.split(b, ")", parts: 2))
  end)
  |> IO.inspect(label: "split3", charlists: :as_lists)
  |> Enum.map(fn
    [a, b] -> [a, AOC.parse_integer(b), nil]
    [a, b, c] -> [a, AOC.parse_integer(b), c]
  end)
  |> IO.inspect(label: "parse2", charlists: :as_lists)
  |> Enum.map(fn
    [a, b, c] when is_number(a) and is_number(b) -> [a * b, c]
    [_, _, c] -> [nil, c]
  end)
  |> IO.inspect(label: "multiply", charlists: :as_lists)
  |> Enum.map(fn
    [a, nil] -> [a, nil]
    [a, b] -> [a, AOC.doOrDont(b)]
  end)
  |> IO.inspect(label: "doornot", charlists: :as_lists)
  |> Enum.reduce({true, 0}, fn item, {do_, acc} ->
    newDo =
      case item do
        [_, nil] -> do_
        [_, itemDo] -> itemDo
      end

    newAcc =
      case item do
        [nil, _] -> acc
        [a, _] when do_ == true -> acc + a
        _ -> acc
      end

    {newDo, newAcc}
  end)

IO.inspect(part2, label: "part2", charlists: :as_lists)

part2b =
  content
  |> String.split("")
  # {do or dont?, do or dont string, mulsting, inmul, mul, result}
  |> Enum.reduce({true, "", "", false, 0, 0}, fn item, {do_, dostr, mulstr, inmul, mul, res} ->
    newDo =
      case dostr do
        "do()" -> true
        "don't()" -> false
        _ -> dostr
      end

    newInMul =
      case mulstr do
        "mul(" -> true
        _ -> inmul
      end

    newDoStr =
      case {dostr, item} do
        {_, "d"} -> "d"
        {"d", "o"} -> "do"
        {"do", "("} -> "do("
        {"do(", "()"} -> "do()"
        {"do", "n"} -> "don"
        {"don", "'"} -> "don'"
        {"don'", "t"} -> "don't"
        {"don't", "("} -> "don't("
        {"don't(", ")"} -> "don't()"
        _ -> dostr
      end

    newMulStr =
      case {mulstr, item, inMul} do
        {_, "m", _} -> "m"
        {"m", "u", _} -> "mu"
        {"mu", "l", _} -> "mul"
        {"mul", "(", _} -> "mul("
        {_, _, false} -> ""
        _ -> mulstr
      end

    # ... enough >D idk anymore
  end)

# well, this is still buggy :( sample is OK, input is not
