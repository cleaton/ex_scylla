defmodule ExScylla.Types.PreparedMetadata do
  alias ExScylla.Types.PartitionKeyIndex
  alias ExScylla.Types.ColumnSpec
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/frame/response/result/struct.PreparedMetadata.html"
  ]

  native_struct col_count: non_neg_integer(),
                pk_indexes: list(PartitionKeyIndex.t()),
                col_specs: list(ColumnSpec.t())
end
