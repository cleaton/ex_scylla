defmodule ExScylla.Types.BatchResult do
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/struct.BatchResult.html"
  ]

  native_struct warnings: list(String.t()),
         tracing_id: binary() | nil
end
