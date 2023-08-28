defmodule ExScylla.SessionBuilder do
  alias ExScylla.Types, as: T
  alias ExScylla.Types.Errors.NewSessionError

  use ExScylla.Macros.Native,
    prefix: :sb,
    docs_rs_path: "/scylla/transport/session_builder/struct.SessionBuilder.html"

  native_f(
    func: :auto_schema_agreement_timeout,
    args: [sb, timeout_ms],
    args_spec: [T.session_builder(), pos_integer()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.auto_schema_agreement_timeout(sb, 5_000)
    iex> true = is_reference(sb)
    """
  )

  native_f_async(
    func: :build,
    args: [sb],
    args_spec: [T.session_builder()],
    return_spec: {:ok, T.session()} | {:error, NewSessionError.t()},
    doc_example: """
    iex> sb = SessionBuilder.new()
    ...>      |> SessionBuilder.known_node("127.0.0.1:9042")
    iex> {:ok, session} = SessionBuilder.build(sb)
    iex> true = is_reference(session)
    """
  )

  native_f(
    func: :compression,
    args: [sb, compression],
    args_spec: [T.session_builder(), T.transport_compression()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.compression(sb, :lz4)
    iex> sb = SessionBuilder.compression(sb, :snappy)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :connection_timeout,
    args: [sb, timeout_ms],
    args_spec: [T.session_builder(), pos_integer()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.connection_timeout(sb, 5_000)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :default_consistency,
    args: [sb, consistency],
    args_spec: [T.session_builder(), T.consistency()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.default_consistency(sb, :quorum)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :disallow_shard_aware_port,
    args: [sb, disallow],
    args_spec: [T.session_builder(), boolean()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.disallow_shard_aware_port(sb, true)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :fetch_schema_metadata,
    args: [sb, fetch],
    args_spec: [T.session_builder(), boolean()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.fetch_schema_metadata(sb, true)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :keepalive_interval,
    args: [sb, interval_ms],
    args_spec: [T.session_builder(), pos_integer()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.keepalive_interval(sb, 5_000)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :known_node,
    args: [sb, hostname],
    args_spec: [T.session_builder(), String.t()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.known_node(sb, "127.0.0.1:9042")
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :known_node_addr,
    args: [sb, node_addr],
    args_spec: [T.session_builder(), {:inet.ip_address(), :inet.port_number()}],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.known_node_addr(sb, {{127,0,0,1}, 9042})
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :known_nodes,
    args: [sb, hostnames],
    args_spec: [T.session_builder(), [String.t()]],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.known_nodes(sb, ["127.0.0.1:9042", "127.0.0.2:9042"])
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :known_nodes_addr,
    args: [sb, node_addrs],
    args_spec: [T.session_builder(), [{:inet.ip_address(), :inet.port_number()}]],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.known_nodes_addr(sb, [{{127,0,0,1}, 9042}, {{127,0,0,1}, 9042}])
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :load_balancing,
    args: [sb, policy],
    args_spec: [T.session_builder(), T.load_balancing_policy()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> policy = %RoundRobinPolicy{token_aware: true}
    iex> sb = SessionBuilder.load_balancing(sb, policy)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :new,
    args: [],
    args_spec: [],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :no_auto_schema_agreement,
    args: [sb],
    args_spec: [T.session_builder()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.no_auto_schema_agreement(sb)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :pool_size,
    args: [sb, size],
    args_spec: [T.session_builder(), T.pool_size()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.pool_size(sb, {:per_shard, 3})
    iex> sb = SessionBuilder.pool_size(sb, {:per_host, 10})
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :retry_policy,
    args: [sb, retry_policy],
    args_spec: [T.session_builder(), T.retry_policy()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.retry_policy(sb, :default_retry_policy)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :schema_agreement_interval,
    args: [sb, interval_ms],
    args_spec: [T.session_builder(), pos_integer()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> interval_ms = 5_000
    iex> sb = SessionBuilder.schema_agreement_interval(sb, interval_ms)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :speculative_execution,
    args: [sb, policy],
    args_spec: [T.session_builder(), T.speculative_execution_policy()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> se = %SimpleSpeculativeExecutionPolicy{max_retry_count: 10, retry_interval_ms: 5_000}
    iex> sb = SessionBuilder.speculative_execution(sb, se)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :tcp_nodelay,
    args: [sb, nodelay],
    args_spec: [T.session_builder(), boolean()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> sb = SessionBuilder.tcp_nodelay(sb, true)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :use_keyspace,
    args: [sb, keyspace_name, case_sensitive],
    args_spec: [T.session_builder(), String.t(), boolean()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> case_sensitive = true
    iex> sb = SessionBuilder.use_keyspace(sb, "my_keyspace", case_sensitive)
    iex> true = is_reference(sb)
    """
  )

  native_f(
    func: :user,
    args: [sb, username, passwd],
    args_spec: [T.session_builder(), String.t(), String.t()],
    return_spec: T.session_builder(),
    doc_example: """
    iex> sb = SessionBuilder.new()
    iex> {username, passwd} = {"user", "myS3cr3tp@ssw0rd"}
    iex> sb = SessionBuilder.user(sb, username, passwd)
    iex> true = is_reference(sb)
    """
  )
end
