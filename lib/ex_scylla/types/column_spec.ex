defmodule ExScylla.Types.ColumnSpec do
  alias ExScylla.Types.ScyllaTableSpec

  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/frame/response/result/struct.ColumnSpec.html"

  native_struct(
    name: String.t(),
    table_spec: ScyllaTableSpec.t(),
    typ: atom()
  )
end
