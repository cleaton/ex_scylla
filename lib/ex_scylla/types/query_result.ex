defmodule ExScylla.Types.QueryResult do
  alias ExScylla.Types.Row
  alias ExScylla.Types.QueryResultRaw
  # alias ExScylla.Types.ScyllaColumnSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/transport/query_result/struct.QueryResult.html"
  ]

  native_struct rows: binary() | list(Row.t()) | nil,
         rows_count: non_neg_integer() | nil,
         column_types: list(term()),
         warnings: list(String.t()),
         tracing_id: binary() | nil,
         paging_state: binary() | nil,
         serialized_size: non_neg_integer()

  def decode(%__MODULE__{rows: rows_bin, rows_count: rows_count, column_types: types} = res) do
    decoded_rows = if is_binary(rows_bin) do
      decode_rows(rows_bin, rows_count, types)
    else
      rows_bin
    end

    %{res | rows: decoded_rows}
  end

  def decode_raw(%__MODULE__{rows: rows_bin, rows_count: rows_count, column_types: types, warnings: warnings, tracing_id: tracing_id, paging_state: paging_state, serialized_size: serialized_size}) do
    decoded_rows = if is_binary(rows_bin) do
      decode_rows_raw(rows_bin, rows_count, types)
    else
      rows_bin
    end

    %QueryResultRaw{
      rows: decoded_rows,
      rows_count: rows_count,
      column_types: types,
      warnings: warnings,
      tracing_id: tracing_id,
      paging_state: paging_state,
      serialized_size: serialized_size
    }
  end

  def decode_rows(bin, count, types), do: decode_rows(bin, count, types, [])
  def decode_rows(_bin, 0, _types, acc), do: Enum.reverse(acc)
  def decode_rows(bin, count, types, acc) do
    {row_cols, rest} = decode_row(bin, types)
    decode_rows(rest, count - 1, types, [%Row{columns: row_cols} | acc])
  end

  def decode_rows_raw(bin, count, types), do: decode_rows_raw(bin, count, types, [])
  def decode_rows_raw(_bin, 0, _types, acc), do: Enum.reverse(acc)
  def decode_rows_raw(bin, count, types, acc) do
    {row_cols, rest} = decode_row_raw(bin, types)
    decode_rows_raw(rest, count - 1, types, [row_cols | acc])
  end

  def decode_row(bin, types), do: decode_row(bin, types, [])
  def decode_row(bin, [], acc), do: {Enum.reverse(acc), bin}
  def decode_row(<<-1::signed-size(32), rest::binary>>, [_type | types], acc) do
    decode_row(rest, types, [nil | acc])
  end
  def decode_row(<<len::signed-size(32), val_bin::binary-size(len), rest::binary>>, [type | types], acc) do
    val = case type do
      :int ->
        <<v::signed-size(32)>> = val_bin
        v
      :big_int ->
        <<v::signed-size(64)>> = val_bin
        v
      :text -> val_bin
      :ascii -> val_bin
      :varchar -> val_bin
      :blob -> val_bin
      :boolean ->
        case val_bin do
          <<0>> -> false
          <<1>> -> true
        end
      :float ->
        <<v::float-size(32)>> = val_bin
        v
      :double ->
        <<v::float-size(64)>> = val_bin
        v
      :timestamp ->
        <<v::signed-size(64)>> = val_bin
        v
      :uuid ->
        case val_bin do
          <<u1::binary-size(4), u2::binary-size(2), u3::binary-size(2), u4::binary-size(2), u5::binary-size(6)>> ->
            "#{Base.encode16(u1, case: :lower)}-#{Base.encode16(u2, case: :lower)}-#{Base.encode16(u3, case: :lower)}-#{Base.encode16(u4, case: :lower)}-#{Base.encode16(u5, case: :lower)}"
          _ ->
            val_bin
        end
      :timeuuid ->
        case val_bin do
          <<u1::binary-size(4), u2::binary-size(2), u3::binary-size(2), u4::binary-size(2), u5::binary-size(6)>> ->
            "#{Base.encode16(u1, case: :lower)}-#{Base.encode16(u2, case: :lower)}-#{Base.encode16(u3, case: :lower)}-#{Base.encode16(u4, case: :lower)}-#{Base.encode16(u5, case: :lower)}"
          _ ->
            val_bin
        end
      :inet ->
        case byte_size(val_bin) do
          4 ->
            <<a, b, c, d>> = val_bin
            {a, b, c, d}
          16 ->
            <<a::16, b::16, c::16, d::16, e::16, f::16, g::16, h::16>> = val_bin
            {a, b, c, d, e, f, g, h}
          _ ->
            val_bin
        end
      _ ->
        ExScylla.CQLTypes.decode_value(val_bin, type)
    end
    decode_row(rest, types, [{type, val} | acc])
  end

  def decode_row_raw(bin, types), do: decode_row_raw(bin, types, [])
  def decode_row_raw(bin, [], acc), do: {Enum.reverse(acc), bin}
  def decode_row_raw(<<-1::signed-size(32), rest::binary>>, [_type | types], acc) do
    decode_row_raw(rest, types, [nil | acc])
  end
  def decode_row_raw(<<len::signed-size(32), val_bin::binary-size(len), rest::binary>>, [type | types], acc) do
    val = case type do
      :int ->
        <<v::signed-size(32)>> = val_bin
        v
      :big_int ->
        <<v::signed-size(64)>> = val_bin
        v
      :text -> val_bin
      :ascii -> val_bin
      :varchar -> val_bin
      :blob -> val_bin
      :boolean ->
        case val_bin do
          <<0>> -> false
          <<1>> -> true
        end
      :float ->
        <<v::float-size(32)>> = val_bin
        v
      :double ->
        <<v::float-size(64)>> = val_bin
        v
      :timestamp ->
        <<v::signed-size(64)>> = val_bin
        v
      :uuid ->
        case val_bin do
          <<u1::binary-size(4), u2::binary-size(2), u3::binary-size(2), u4::binary-size(2), u5::binary-size(6)>> ->
            "#{Base.encode16(u1, case: :lower)}-#{Base.encode16(u2, case: :lower)}-#{Base.encode16(u3, case: :lower)}-#{Base.encode16(u4, case: :lower)}-#{Base.encode16(u5, case: :lower)}"
          _ ->
            val_bin
        end
      :timeuuid ->
        case val_bin do
          <<u1::binary-size(4), u2::binary-size(2), u3::binary-size(2), u4::binary-size(2), u5::binary-size(6)>> ->
            "#{Base.encode16(u1, case: :lower)}-#{Base.encode16(u2, case: :lower)}-#{Base.encode16(u3, case: :lower)}-#{Base.encode16(u4, case: :lower)}-#{Base.encode16(u5, case: :lower)}"
          _ ->
            val_bin
        end
      :inet ->
        case byte_size(val_bin) do
          4 ->
            <<a, b, c, d>> = val_bin
            {a, b, c, d}
          16 ->
            <<a::16, b::16, c::16, d::16, e::16, f::16, g::16, h::16>> = val_bin
            {a, b, c, d, e, f, g, h}
          _ ->
            val_bin
        end
      _ ->
        ExScylla.CQLTypes.decode_value(val_bin, type)
    end
    decode_row_raw(rest, types, [val | acc])
  end
end
