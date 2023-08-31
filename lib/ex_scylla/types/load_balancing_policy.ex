defmodule ExScylla.Types.LoadBalancingPolicy do
  use ExScylla.Macros.Native,
    # TODO fix doc path
    docs_rs_path: "/scylla/transport/load_balancing/struct.RoundRobinPolicy.html"

  # TODO is there a way i can have this have defaults, so we only need to pass in the values we want to change?
  native_struct(
    datacenter: String.t(),
    rack: String.t(),
    is_token_aware: boolean(),
    permit_dc_failover: boolean(),
    enable_shuffling_replicas: boolean()
    # TODO
    # latency_awareness ...
  )
end
