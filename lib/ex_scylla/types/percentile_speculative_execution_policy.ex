defmodule ExScylla.Types.PercentileSpeculativeExecutionPolicy do
  use ExScylla.Macros.Native,
    docs_rs_path:
      "/scylla/policies/speculative_execution/struct.PercentileSpeculativeExecutionPolicy.html"

  native_struct(
    max_retry_count: pos_integer(),
    percentile: float()
  )
end
