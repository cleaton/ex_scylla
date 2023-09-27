defmodule ExScylla.Types.QueryResult do
  alias ExScylla.Types.ScyllaRow
  # alias ExScylla.Types.ScyllaColumnSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/transport/query_result/struct.QueryResult.html"
  ]



  native_struct rows: list(ScyllaRow.t()) | nil,
         paging_state: binary() | nil,
         tracing_id: binary() | nil,
         warnings: list(String.t()),
         # TODO: find way to add without overhead. Make it optional?
        #col_specs: list(ScyllaColumnSpec.t()),
         serialized_size: non_neg_integer()
end
