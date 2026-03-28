defmodule ExScylla.Types.Metrics do
  defstruct [
    :errors_num,
    :queries_num,
    :errors_iter_num,
    :queries_iter_num,
    :retries_num,
    :mean_rate,
    :one_minute_rate,
    :five_minute_rate,
    :fifteen_minute_rate,
    :total_connections,
    :connection_timeouts,
    :request_timeouts,
    :latency_avg_ms,
    :latency_99_percentile_ms
  ]

  @type t :: %__MODULE__{
          errors_num: non_neg_integer(),
          queries_num: non_neg_integer(),
          errors_iter_num: non_neg_integer(),
          queries_iter_num: non_neg_integer(),
          retries_num: non_neg_integer(),
          mean_rate: float(),
          one_minute_rate: float(),
          five_minute_rate: float(),
          fifteen_minute_rate: float(),
          total_connections: non_neg_integer(),
          connection_timeouts: non_neg_integer(),
          request_timeouts: non_neg_integer(),
          latency_avg_ms: non_neg_integer() | nil,
          latency_99_percentile_ms: non_neg_integer() | nil
        }
end
