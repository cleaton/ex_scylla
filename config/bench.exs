import Config

# Optional overrides for benchmark runs:
#   EX_SCYLLA_TEST_SMP=4         number of ScyllaDB shards (read by ExScylla.TestSupport)
#   EX_SCYLLA_TOKIO_WORKERS=4    ExScylla's Tokio worker thread count
if workers = System.get_env("EX_SCYLLA_TOKIO_WORKERS") do
  config :ex_scylla, tokio_worker_threads: String.to_integer(workers)
end
