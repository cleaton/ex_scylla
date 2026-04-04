defmodule ExScylla.Types.QueryResult do
  alias ExScylla.Types.Row
  alias ExScylla.Types.QueryResultRaw
  # alias ExScylla.Types.ScyllaColumnSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/response/query_result/struct.QueryResult.html"
  ]

  native_struct rows: binary() | list(Row.t()) | nil,
         rows_count: non_neg_integer() | nil,
         column_types: list(ExScylla.Types.column_type()),
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
      # Ultra-optimized row decoding for raw API
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
    val = ExScylla.CQLTypes.decode_value(val_bin, type)
    decode_row(rest, types, [{type, val} | acc])
  end

  def decode_row_raw(bin, types), do: decode_row_raw(bin, types, [])
  def decode_row_raw(bin, [], acc), do: {Enum.reverse(acc), bin}
  def decode_row_raw(<<-1::signed-size(32), rest::binary>>, [_type | types], acc) do
    decode_row_raw(rest, types, [nil | acc])
  end
  def decode_row_raw(<<len::signed-size(32), val_bin::binary-size(len), rest::binary>>, [type | types], acc) do
    val = ExScylla.CQLTypes.decode_value(val_bin, type)
    decode_row_raw(rest, types, [val | acc])
  end
end
