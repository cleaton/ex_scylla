defmodule ExScylla.Types.QueryResult do
  alias ExScylla.Types.ScyllaRow
  alias ExScylla.Types.ScyllaColumnSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/transport/query_result/struct.QueryResult.html"
  ]

  native_struct col_specs: list(ScyllaColumnSpec.t()),
         paging_state: binary() | nil,
         rows: list(ScyllaRow.t()) | nil,
         tracing_id: binary() | nil,
         warnings: list(String.t())
end
