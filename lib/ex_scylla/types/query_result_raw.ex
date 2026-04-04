defmodule ExScylla.Types.QueryResultRaw do
  @moduledoc """
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
