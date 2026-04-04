defmodule ExScylla.Types.PartitionKeyIndex do
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/frame/response/result/struct.PartitionKeyIndex.html"

  native_struct(
    index: non_neg_integer(),
    sequence: non_neg_integer()
  )
end
