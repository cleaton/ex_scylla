defmodule ExScylla.CqlTypes do
  @moduledoc """
  Pure Elixir codec for ScyllaDB types.
  """

  @spec decode_value(binary(), atom() | tuple()) :: term()
  def decode_value(bin, type)

  def decode_value(bin, :int) do
    <<val::signed-size(32)>> = bin
    val
  end

  def decode_value(bin, :big_int) do
    <<val::signed-size(64)>> = bin
    val
  end

  def decode_value(bin, :small_int) do
    <<val::signed-size(16)>> = bin
    val
  end

  def decode_value(bin, :tiny_int) do
    <<val::signed-size(8)>> = bin
    val
  end

  def decode_value(bin, :boolean) do
    case bin do
      <<0>> -> false
      <<1>> -> true
    end
  end

  def decode_value(bin, :float) do
    <<val::float-size(32)>> = bin
    val
  end

  def decode_value(bin, :double) do
    <<val::float-size(64)>> = bin
    val
  end

  def decode_value(bin, :text), do: bin
  def decode_value(bin, :ascii), do: bin
  def decode_value(bin, :blob), do: bin

  def decode_value(bin, :uuid) do
    <<u1::binary-size(4), u2::binary-size(2), u3::binary-size(2), u4::binary-size(2),
      u5::binary-size(6)>> = bin

    "#{Base.encode16(u1, case: :lower)}-#{Base.encode16(u2, case: :lower)}-#{Base.encode16(u3, case: :lower)}-#{Base.encode16(u4, case: :lower)}-#{Base.encode16(u5, case: :lower)}"
  end

  def decode_value(bin, :timeuuid), do: decode_value(bin, :uuid)

  def decode_value(bin, :inet) do
    case byte_size(bin) do
      4 ->
        <<a, b, c, d>> = bin
        {a, b, c, d}

      16 ->
        <<a::16, b::16, c::16, d::16, e::16, f::16, g::16, h::16>> = bin
        {a, b, c, d, e, f, g, h}
    end
  end

  def decode_value(bin, :date) do
    <<val::unsigned-size(32)>> = bin
    val
  end

  def decode_value(bin, :time) do
    <<val::signed-size(64)>> = bin
    val
  end

  def decode_value(bin, :timestamp) do
    <<val::signed-size(64)>> = bin
    val
  end

  def decode_value(bin, :counter) do
    <<val::signed-size(64)>> = bin
    val
  end

  def decode_value(bin, :varint) do
    decode_varint(bin) |> Integer.to_string()
  end

  def decode_value(bin, :decimal) do
    <<scale::signed-size(32), unscaled_bin::binary>> = bin
    unscaled = decode_varint(unscaled_bin)

    # Simple string formatting for the test
    str = Integer.to_string(unscaled)

    if scale > 0 do
      len = byte_size(str)

      if len > scale do
        {int_part, frac_part} = String.split_at(str, len - scale)
        "#{int_part}.#{frac_part}"
      else
        pad = String.duplicate("0", scale - len + 1)
        "0.#{String.slice(pad <> str, 1..-1//1)}"
      end
    else
      str
    end
  end

  def decode_value(bin, {:list, inner_type}) do
    <<count::signed-size(32), rest::binary>> = bin
    decode_collection(rest, count, inner_type, [])
  end

  def decode_value(bin, {:set, inner_type}) do
    <<count::signed-size(32), rest::binary>> = bin
    decode_collection(rest, count, inner_type, [])
  end

  def decode_value(bin, {:map, {key_type, val_type}}) do
    <<count::signed-size(32), rest::binary>> = bin
    decode_map(rest, count, key_type, val_type, [])
  end

  def decode_value(bin, {:tuple, types}) do
    decode_tuple(bin, types, [])
  end

  def decode_value(bin, {:user_defined_type, fields}) do
    decode_udt(bin, fields, %{})
  end

  def decode_value(bin, _unknown_type), do: bin

  defp decode_varint(bin) do
    bits = byte_size(bin) * 8
    <<val::signed-size(bits)>> = bin
    val
  end

  defp decode_collection(_bin, 0, _type, acc), do: Enum.reverse(acc)

  defp decode_collection(<<len::signed-size(32), rest::binary>>, count, type, acc) do
    case len do
      -1 ->
        decode_collection(rest, count - 1, type, [nil | acc])

      _ ->
        <<val_bin::binary-size(len), remaining::binary>> = rest
        val = decode_value(val_bin, type)
        decode_collection(remaining, count - 1, type, [val | acc])
    end
  end

  defp decode_map(_bin, 0, _ktype, _vtype, acc), do: Map.new(acc)

  defp decode_map(<<klen::signed-size(32), rest1::binary>>, count, ktype, vtype, acc) do
    {k_val, rest2} =
      case klen do
        -1 ->
          {nil, rest1}

        _ ->
          <<k_bin::binary-size(klen), rem1::binary>> = rest1
          {decode_value(k_bin, ktype), rem1}
      end

    <<vlen::signed-size(32), rest3::binary>> = rest2

    {v_val, rest4} =
      case vlen do
        -1 ->
          {nil, rest3}

        _ ->
          <<v_bin::binary-size(vlen), rem2::binary>> = rest3
          {decode_value(v_bin, vtype), rem2}
      end

    decode_map(rest4, count - 1, ktype, vtype, [{k_val, v_val} | acc])
  end

  defp decode_tuple(_bin, [], acc), do: Enum.reverse(acc) |> List.to_tuple()

  defp decode_tuple(<<len::signed-size(32), rest::binary>>, [type | types], acc) do
    case len do
      -1 ->
        decode_tuple(rest, types, [nil | acc])

      _ ->
        <<val_bin::binary-size(len), remaining::binary>> = rest
        val = decode_value(val_bin, type)
        decode_tuple(remaining, types, [val | acc])
    end
  end

  defp decode_tuple(<<>>, _types, acc), do: Enum.reverse(acc) |> List.to_tuple()

  defp decode_udt(_bin, [], acc), do: acc

  defp decode_udt(<<len::signed-size(32), rest::binary>>, [{name, type} | fields], acc) do
    case len do
      -1 ->
        decode_udt(rest, fields, Map.put(acc, name, nil))

      _ ->
        <<val_bin::binary-size(len), remaining::binary>> = rest
        val = decode_value(val_bin, type)
        decode_udt(remaining, fields, Map.put(acc, name, val))
    end
  end

  defp decode_udt(<<>>, _fields, acc), do: acc
end
