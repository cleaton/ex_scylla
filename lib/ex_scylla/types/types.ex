defmodule ExScylla.Types do
  @moduledoc """
  This module defines the core type hierarchy and aliases used throughout the `ExScylla` library.
  It provides Elixir type specs that map to the underlying Rust Scylla driver types.
  """
  alias ExScylla.Types.RoundRobinPolicy
  alias ExScylla.Types.DcAwareRoundRobinPolicy
  alias ExScylla.Types.SimpleSpeculativeExecutionPolicy
  alias ExScylla.Types.PercentileSpeculativeExecutionPolicy
  alias ExScylla.Types.CqlDuration
  alias ExScylla.Types.UserDefinedType
  alias ExScylla.Types.UserDefinedColumnType

  alias ExScylla.Types.Metrics
  alias ExScylla.Types.ClusterState
  alias ExScylla.Types.TracingInfo

  @opaque batch :: reference()
  @type ts_micros :: integer()
  @type duration_ms :: non_neg_integer()
  @opaque query :: reference()
  @opaque session :: reference()
  @opaque execution_profile_handle :: reference()
  @opaque execution_profile :: reference()
  @opaque execution_profile_builder :: reference()
  @opaque prepared_statement :: reference()
  @type consistency ::
          :any | :one | :two | :three | :quorum | :all | :local_quorum | :each_quorum | :local_one
  @opaque session_builder :: reference()
  @opaque default_policy_builder :: reference()
  @opaque latency_awareness_builder :: reference()
  @opaque load_balancing_policy_resource :: reference()
  @type transport_compression :: :lz4 | :snappy
  @type load_balancing_policy :: RoundRobinPolicy.t() | DcAwareRoundRobinPolicy.t()
  @type pool_size :: {:per_host, pos_integer()} | {:per_shard, pos_integer()}
  @type retry_policy :: :default_retry_policy | :fall_through_retry_policy
  @type speculative_execution_policy ::
          SimpleSpeculativeExecutionPolicy.t() | PercentileSpeculativeExecutionPolicy.t()
  @type serial_consistency :: :serial | :local_serial
  @type paging_state :: binary()
  @type batch_type :: :logged | :unlogged | :counter
  @type uuid :: binary()
  @type metrics :: Metrics.t()
  @type cluster_state :: ClusterState.t()
  @type tracing_info :: TracingInfo.t()
  @type msg :: String.t()
  @type parse_error :: {:error, {:parse_value, msg()}}

  @type value ::
          {:ascii, String.t()}
          | {:boolean, boolean()}
          | {:blob, binary()}
          | {:counter, integer()}
          # TODO: Better way to represent decimal in elixir?
          | {:decimal, String.t()}
          # Days since -5877641-06-23 i.e. 2^31 days before unix epoch
          | {:date, non_neg_integer()}
          | {:double, float()}
          | {:duration, CqlDuration.t()}
          | :empty
          | {:float, float()}
          | {:int, integer()}
          | {:big_int, integer()}
          | {:text, String.t()}
          # Milliseconds since unix epoch
          | {:timestamp, integer()}
          | {:inet, :inet.ip_address()}
          | {:list, list(value())}
          | {:map, list({value(), value()})}
          | {:set, list(value())}
          | {:user_defined_type, UserDefinedType.t()}
          | {:small_int, integer()}
          | {:tiny_int, integer()}
          # Nanoseconds since midnight
          | {:time, integer()}
          | {:timeuuid, binary()}
          | {:tuple, list(value() | nil)}
          | {:uuid, binary()}
          # TODO: Better way to represent varint in elixir?
          | {:varint, String.t()}
  @type values :: list(value())

  @type column_type ::
          {:custom, String.t()}
          | :ascii
          | :boolean
          | :blob
          | :counter
          | :date
          | :decimal
          | :double
          | :duration
          | :float
          | :int
          | :big_int
          | :text
          | :timestamp
          | :inet
          | {:list, column_type()}
          | {:map, {column_type(), column_type()}}
          | {:set, column_type()}
          | {:user_defined_type, UserDefinedColumnType.t()}
          | :small_int
          | :tiny_int
          | :time
          | :timeuuid
          | {:tuple, list(column_type())}
          | :uuid
          | :varint
end
