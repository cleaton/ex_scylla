defmodule ExScylla.Types.QueryResultRaw do
  alias ExScylla.Macros.Native

  @moduledoc """
  Elixir-only decoded shape for `query_raw` / `execute_raw` performance paths (not a 1:1 Rust struct).
  See Rust [`QueryResult`](https://docs.rs/scylla/#{Native.scylla_version()}/scylla/response/query_result/struct.QueryResult.html) for the driver type this wraps at the protocol level.

  A raw representation of a query result for maximum performance.
  Instead of `%ExScylla.Types.Row{columns: [{:type, value}]}`, the `rows` field
  contains a simple list of lists of values `[[value]]`.
  """

  @enforce_keys [
    :rows,
    :rows_count,
    :column_types,
    :warnings,
    :tracing_id,
    :paging_state,
    :serialized_size
  ]
  defstruct [
    :rows,
    :rows_count,
    :column_types,
    :warnings,
    :tracing_id,
    :paging_state,
    :serialized_size
  ]

  @type t :: %__MODULE__{
          rows: list(list(term())) | nil,
          rows_count: non_neg_integer() | nil,
          column_types: list(term()),
          warnings: list(String.t()),
          tracing_id: binary() | nil,
          paging_state: binary() | nil,
          serialized_size: non_neg_integer()
        }
end
