import Config

config :erlcass,
  keyspace: "load_test_erlcass",
  cluster_options: [
    {:contact_points, "127.0.0.1"},
    {:latency_aware_routing, true},
    {:token_aware_routing, true},
    {:number_threads_io, 4},
    {:queue_size_io, 128000},
    {:core_connections_host, 1},
    {:tcp_nodelay, true},
    {:tcp_keepalive, {true, 60}},
    {:connect_timeout, 5000},
    {:request_timeout, 5000},
    {:retry_policy, {:default, true}}
  ]
