defmodule ExScylla.Types.SimpleSpeculativeExecutionPolicy do
  use ExScylla.Macros.Native,
    docs_rs_path:
      "/scylla/transport/speculative_execution/struct.SimpleSpeculativeExecutionPolicy.html"

  native_struct(
    max_retry_count: pos_integer(),
    retry_interval_ms: pos_integer()
  )
end
