defmodule ExScylla.Types.QueryResult do
  alias ExScylla.Types.Row
  alias ExScylla.Types.QueryResultRaw
  # alias ExScylla.Types.ScyllaColumnSpec
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/response/query_result/struct.QueryResult.html"

  native_struct(
    rows: binary() | list(Row.t()) | nil,
    rows_count: non_neg_integer() | nil,
    column_types: list(ExScylla.Types.column_type()),
    warnings: list(String.t()),
    tracing_id: binary() | nil,
    paging_state: binary() | nil,
    serialized_size: non_neg_integer()
  )

  def decode(%__MODULE__{rows: rows_bin, rows_count: rows_count, column_types: types} = res) do
    decoded_rows =
      if is_binary(rows_bin) do
        decode_rows(rows_bin, rows_count, types)
      else
        rows_bin
      end

    %{res | rows: decoded_rows}
  end

  def decode_raw(%__MODULE__{
        rows: rows_bin,
        rows_count: rows_count,
        column_types: types,
        warnings: warnings,
        tracing_id: tracing_id,
        paging_state: paging_state,
        serialized_size: serialized_size
      }) do
    decoded_rows =
      if is_binary(rows_bin) do
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

  # Public helpers. The recursion lives in private functions so the compiler can keep
  # one binary match context across cells instead of creating a sub-binary per call.
  def decode_rows(bin, count, types), do: rows_typed(bin, count, types, [])
  def decode_rows(bin, count, types, acc), do: rows_typed(bin, count, types, acc)
  def decode_rows_raw(bin, count, types), do: rows_raw(bin, count, types, [])
  def decode_rows_raw(bin, count, types, acc), do: rows_raw(bin, count, types, acc)
  def decode_row(bin, types), do: row_typed(bin, types, [])
  def decode_row(bin, types, acc), do: row_typed(bin, types, acc)
  def decode_row_raw(bin, types), do: row_raw(bin, types, [])
  def decode_row_raw(bin, types, acc), do: row_raw(bin, types, acc)

  defp rows_typed(_bin, 0, _types, acc), do: Enum.reverse(acc)

  defp rows_typed(bin, count, types, acc) do
    {row_cols, rest} = row_typed(bin, types, [])
    rows_typed(rest, count - 1, types, [%Row{columns: row_cols} | acc])
  end

  defp rows_raw(_bin, 0, _types, acc), do: Enum.reverse(acc)

  defp rows_raw(bin, count, types, acc) do
    {row_cols, rest} = row_raw(bin, types, [])
    rows_raw(rest, count - 1, types, [row_cols | acc])
  end

  defp row_typed(bin, [], acc), do: {Enum.reverse(acc), bin}

  defp row_typed(<<-1::signed-size(32), rest::binary>>, [_type | types], acc) do
    row_typed(rest, types, [nil | acc])
  end

  # Fast paths: common types are matched straight out of the row binary, without a
  # per-cell sub-binary or a call to CqlTypes.decode_value/2. Anything else (other
  # types, empty values, unexpected lengths) falls through to the generic clause.
  defp row_typed(<<4::32, v::signed-32, rest::binary>>, [:int | types], acc),
    do: row_typed(rest, types, [{:int, v} | acc])

  defp row_typed(<<8::32, v::signed-64, rest::binary>>, [:big_int | types], acc),
    do: row_typed(rest, types, [{:big_int, v} | acc])

  defp row_typed(<<8::32, v::signed-64, rest::binary>>, [:timestamp | types], acc),
    do: row_typed(rest, types, [{:timestamp, v} | acc])

  defp row_typed(<<8::32, v::signed-64, rest::binary>>, [:time | types], acc),
    do: row_typed(rest, types, [{:time, v} | acc])

  defp row_typed(<<8::32, v::signed-64, rest::binary>>, [:counter | types], acc),
    do: row_typed(rest, types, [{:counter, v} | acc])

  defp row_typed(<<2::32, v::signed-16, rest::binary>>, [:small_int | types], acc),
    do: row_typed(rest, types, [{:small_int, v} | acc])

  defp row_typed(<<1::32, v::signed-8, rest::binary>>, [:tiny_int | types], acc),
    do: row_typed(rest, types, [{:tiny_int, v} | acc])

  defp row_typed(<<4::32, v::unsigned-32, rest::binary>>, [:date | types], acc),
    do: row_typed(rest, types, [{:date, v} | acc])

  defp row_typed(<<8::32, v::float-64, rest::binary>>, [:double | types], acc),
    do: row_typed(rest, types, [{:double, v} | acc])

  defp row_typed(<<4::32, v::float-32, rest::binary>>, [:float | types], acc),
    do: row_typed(rest, types, [{:float, v} | acc])

  defp row_typed(<<len::signed-32, v::binary-size(len), rest::binary>>, [:text | types], acc),
    do: row_typed(rest, types, [{:text, v} | acc])

  defp row_typed(<<len::signed-32, v::binary-size(len), rest::binary>>, [:ascii | types], acc),
    do: row_typed(rest, types, [{:ascii, v} | acc])

  defp row_typed(<<len::signed-32, v::binary-size(len), rest::binary>>, [:blob | types], acc),
    do: row_typed(rest, types, [{:blob, v} | acc])

  defp row_typed(<<1::32, b, rest::binary>>, [:boolean | types], acc) when b in [0, 1],
    do: row_typed(rest, types, [{:boolean, b == 1} | acc])

  defp row_typed(
         <<len::signed-size(32), val_bin::binary-size(len), rest::binary>>,
         [type | types],
         acc
       ) do
    # An empty cell that decodes to nil is reported like NULL: a bare nil.
    cell =
      case ExScylla.CqlTypes.decode_value(val_bin, type) do
        nil -> nil
        val -> {type, val}
      end

    row_typed(rest, types, [cell | acc])
  end

  defp row_raw(bin, [], acc), do: {Enum.reverse(acc), bin}

  defp row_raw(<<-1::signed-size(32), rest::binary>>, [_type | types], acc) do
    row_raw(rest, types, [nil | acc])
  end

  # Fast paths: common types are matched straight out of the row binary, without a
  # per-cell sub-binary or a call to CqlTypes.decode_value/2. Anything else (other
  # types, empty values, unexpected lengths) falls through to the generic clause.
  defp row_raw(<<4::32, v::signed-32, rest::binary>>, [:int | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<8::32, v::signed-64, rest::binary>>, [:big_int | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<8::32, v::signed-64, rest::binary>>, [:timestamp | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<8::32, v::signed-64, rest::binary>>, [:time | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<8::32, v::signed-64, rest::binary>>, [:counter | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<2::32, v::signed-16, rest::binary>>, [:small_int | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<1::32, v::signed-8, rest::binary>>, [:tiny_int | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<4::32, v::unsigned-32, rest::binary>>, [:date | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<8::32, v::float-64, rest::binary>>, [:double | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<4::32, v::float-32, rest::binary>>, [:float | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<len::signed-32, v::binary-size(len), rest::binary>>, [:text | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(
         <<len::signed-32, v::binary-size(len), rest::binary>>,
         [:ascii | types],
         acc
       ),
       do: row_raw(rest, types, [v | acc])

  defp row_raw(<<len::signed-32, v::binary-size(len), rest::binary>>, [:blob | types], acc),
    do: row_raw(rest, types, [v | acc])

  defp row_raw(<<1::32, b, rest::binary>>, [:boolean | types], acc) when b in [0, 1],
    do: row_raw(rest, types, [b == 1 | acc])

  defp row_raw(
         <<len::signed-size(32), val_bin::binary-size(len), rest::binary>>,
         [type | types],
         acc
       ) do
    val = ExScylla.CqlTypes.decode_value(val_bin, type)
    row_raw(rest, types, [val | acc])
  end
end
