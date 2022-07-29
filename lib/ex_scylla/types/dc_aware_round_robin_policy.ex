defmodule ExScylla.Types.DcAwareRoundRobinPolicy do
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/transport/load_balancing/struct.DcAwareRoundRobinPolicy.html"
  ]

  native_struct local_dc: String.t(),
                token_aware: boolean()
end
