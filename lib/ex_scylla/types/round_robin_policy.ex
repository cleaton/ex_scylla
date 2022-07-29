defmodule ExScylla.Types.RoundRobinPolicy do
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/transport/load_balancing/struct.RoundRobinPolicy.html"
  ]

  native_struct token_aware: boolean()

end
