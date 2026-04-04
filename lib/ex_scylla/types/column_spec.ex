defmodule ExScylla.Types.ColumnSpec do
  alias ExScylla.Types.TableSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla_cql/frame/response/result/struct.ColumnSpec.html"
  ]

  native_struct name: String.t(),
         table_spec: TableSpec.t(),
         typ: atom()
end
