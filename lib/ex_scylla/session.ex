defmodule ExScylla.Session do
  alias ExScylla.Types, as: T
  alias ExScylla.Types.QueryResult
  alias ExScylla.Types.Errors.QueryError
  alias ExScylla.Types.Errors.SerializeValuesError
  alias ExScylla.Types.Token
  alias ExScylla.Types.Metrics
  alias ExScylla.Types.ClusterState
  alias ExScylla.Types.TracingInfo

  use ExScylla.Macros.Native,
    prefix: :s,
    docs_rs_path: "/scylla/client/session/struct.Session.html",
    session_setup: """
    iex> node = Application.get_env(:ex_scylla, :test_node, "127.0.0.1:9042")
    iex> {:ok, session} = SessionBuilder.new()
    ...>                  |> SessionBuilder.known_node(node)
    ...>                  |> SessionBuilder.build()
    """

  defp to_scylla_query(q) when is_binary(q), do: {:string, q}
  defp to_scylla_query(q) when is_reference(q), do: {:query_resource, q}

  native_f(
    func: :calculate_token_for_partition_key,
    args: [session, keyspace, table, partition_key],
    args_spec: [T.session(), String.t(), String.t(), T.values()],
    return_spec: Token.t() | nil | {:error, SerializeValuesError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> alias ExScylla.Types.Token
    iex> values = [{:text, "test"}]
    iex> %Token{value: t} = Session.calculate_token_for_partition_key(session, "test", "session_doc", values)
    iex> true = is_integer(t)
    """
  )

  native_f(
    func: :get_cluster_state,
    args: [session],
    args_spec: [T.session()],
    return_spec: ClusterState.t(),
    example_setup: :session_setup,
    doc_example: """
    iex> alias ExScylla.Types.ClusterState
    iex> %ClusterState{nodes: nodes} = Session.get_cluster_state(session)
    iex> true = is_list(nodes)
    """
  )

  native_f(
    func: :get_default_execution_profile_handle,
    args: [session],
    args_spec: [T.session()],
    return_spec: T.execution_profile_handle(),
    example_setup: :session_setup,
    doc_example: """
    iex> eph = Session.get_default_execution_profile_handle(session)
    iex> true = is_reference(eph)
    """
  )

  native_f(
    func: :get_keyspace,
    args: [session],
    args_spec: [T.session()],
    return_spec: String.t() | nil,
    example_setup: :session_setup,
    doc_example: """
    iex> nil = Session.get_keyspace(session)
    iex> {:ok, _} = Session.query(session, "CREATE KEYSPACE IF NOT EXISTS test_ks WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor': 1};", [])
    iex> :ok = Session.use_keyspace(session, "test_ks", false)
    iex> "test_ks" = Session.get_keyspace(session)
    """
  )

  native_f(
    func: :get_metrics,
    args: [session],
    args_spec: [T.session()],
    return_spec: Metrics.t(),
    example_setup: :session_setup,
    doc_example: """
    iex> alias ExScylla.Types.Metrics
    iex> %Metrics{} = Session.get_metrics(session)
    """
  )

  native_f_async(
    func: :get_tracing_info,
    args: [session, tracing_id],
    args_spec: [T.session(), binary()],
    return_spec: {:ok, TracingInfo.t()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> alias ExScylla.Types.TracingInfo
    iex> # Tracing must be enabled on statement/query
    iex> q = ExScylla.Statement.Query.new("SELECT * FROM test.session_doc") |> ExScylla.Statement.Query.set_tracing(true)
    iex> {:ok, %QueryResult{tracing_id: tid}} = Session.query(session, q, [])
    iex> {:ok, %TracingInfo{}} = Session.get_tracing_info(session, tid)
    """
  )

  native_f_async(
    func: :prepare_batch,
    args: [session, batch],
    args_spec: [T.session(), T.batch()],
    return_spec: {:ok, T.batch()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> alias ExScylla.Statement.Batch
    iex> batch = Batch.new(:unlogged)
    ...>   |> Batch.append_statement("INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)")
    iex> {:ok, prepared_batch} = Session.prepare_batch(session, batch)
    iex> true = is_reference(prepared_batch)
    """
  )

  @doc """
  Returns a stream of rows (`ExScylla.Types.Row.t()`) for the given query and values.
  If an error occurs during stream evaluation, the stream will yield `{:error, reason}` as its final element and then halt.
  """
  @spec query_stream(T.session(), String.t() | T.query(), T.values()) :: Enumerable.t()
  def query_stream(session, query, values) do
    Stream.resource(
      fn -> nil end,
      fn
        :done ->
          {:halt, :done}

        paging_state ->
          case query_paged(session, query, values, paging_state) do
            {:ok, %QueryResult{rows: rows, paging_state: next_paging_state}} ->
              next_state = next_paging_state || :done
              {rows || [], next_state}

            {:error, reason} ->
              {[{:error, reason}], :done}
          end
      end,
      fn _ -> :ok end
    )
  end

  @doc """
  Returns a stream of rows (`ExScylla.Types.Row.t()`) for the given prepared statement and values.
  If an error occurs during stream evaluation, the stream will yield `{:error, reason}` as its final element and then halt.
  """
  @spec execute_stream(T.session(), T.prepared_statement(), T.values()) :: Enumerable.t()
  def execute_stream(session, prepared, values) do
    Stream.resource(
      fn -> nil end,
      fn
        :done ->
          {:halt, :done}

        paging_state ->
          case execute_paged(session, prepared, values, paging_state) do
            {:ok, %QueryResult{rows: rows, paging_state: next_paging_state}} ->
              next_state = next_paging_state || :done
              {rows || [], next_state}

            {:error, reason} ->
              {[{:error, reason}], :done}
          end
      end,
      fn _ -> :ok end
    )
  end

  native_f_async(
    func: :await_schema_agreement,
    args: [session],
    args_spec: [T.session()],
    return_spec: :ok | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> Session.await_schema_agreement(session)
    :ok
    """
  )

  native_f_async(
    func: :await_timed_schema_agreement,
    args: [session, timeout_ms],
    args_spec: [T.session(), pos_integer()],
    return_spec: {:ok, boolean()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> Session.await_timed_schema_agreement(session, 15_000)
    {:ok, true}
    """
  )

  native_f_async(
    func: :batch,
    args: [session, batch, values],
    args_spec: [T.session(), T.batch(), T.values()],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode(res)}
        other -> other
      end,
    example_setup: :session_setup,
    doc_example: """
    iex> batch = Batch.new(:unlogged)
    ...>   |> Batch.append_statement("INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)")
    iex> values = [
    ...>   [{:text, "test"}, {:int, 2}, {:double, 1.0}]
    ...> ]
    iex> {:ok, %QueryResult{}} = Session.batch(session, batch, values)
    """
  )

  native_f_async(
    func: :batch,
    as: :batch_raw,
    args: [session, batch, values],
    args_spec: [T.session(), T.batch(), T.values()],
    return_spec: {:ok, QueryResultRaw.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode_raw(res)}
        other -> other
      end
  )

  native_f(
    func: :calculate_token,
    args: [session, prepared, values],
    args_spec: [T.session(), T.prepared_statement(), T.values()],
    return_spec: Token.t() | nil | {:error, SerializeValuesError.t() | QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.session_doc WHERE a = ?;")
    iex> values = [{:text, "test"}]
    iex> %Token{value: t} = Session.calculate_token(session, ps, values)
    iex> true = is_integer(t)
    """
  )

  native_f_async(
    func: :check_schema_agreement,
    args: [session],
    args_spec: [T.session()],
    return_spec: {:ok, boolean()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, true} = Session.check_schema_agreement(session)
    """
  )

  native_f_async(
    func: :execute_paged,
    args: [session, prepared, values, paging_state],
    args_spec: [T.session(), T.prepared_statement(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode(res)}
        other -> other
      end,
    example_setup: :session_setup,
    doc_example: """
    iex> query = "INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)"
    iex> values = [{:text, "test_execute_paged"}, {:int, 1}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.query(session, query, values)
    iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.session_doc WHERE a = ?;")
    iex> ps = Prepared.set_page_size(ps, 1)
    iex> values = [{:text, "test_execute_paged"}]
    iex> {:ok, %QueryResult{paging_state: pgs}} = Session.execute_paged(session, ps, values, nil)
    iex> true = is_binary(pgs)
    iex> {:ok, %QueryResult{}} = Session.execute_paged(session, ps, values, pgs)
    """
  )

  native_f_async(
    func: :execute_paged,
    as: :execute_raw_paged,
    args: [session, prepared, values, paging_state],
    args_spec: [T.session(), T.prepared_statement(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResultRaw.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode_raw(res)}
        other -> other
      end
  )

  native_f_async(
    func: :execute,
    args: [session, prepared, values],
    args_spec: [T.session(), T.prepared_statement(), T.values()],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode(res)}
        other -> other
      end,
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)")
    iex> values = [{:text, "test"}, {:int, 2}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.execute(session, ps, values)
    """
  )

  native_f_async(
    func: :execute,
    as: :execute_raw,
    args: [session, prepared, values],
    args_spec: [T.session(), T.prepared_statement(), T.values()],
    return_spec: {:ok, QueryResultRaw.t()} | {:error, QueryError.t()},
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode_raw(res)}
        other -> other
      end
  )

  native_f_async(
    func: :fetch_schema_version,
    args: [session],
    args_spec: [T.session()],
    return_spec: {:ok, T.uuid()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, version} = Session.fetch_schema_version(session)
    iex> true = is_binary(version)
    """
  )

  native_f_async(
    func: :prepare,
    args: [session, query],
    args_spec: [T.session(), String.t() | T.query()],
    return_spec: {:ok, T.prepared_statement()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.session_doc WHERE a = ?;")
    iex> true = is_reference(ps)
    """
  )

  native_f_async(
    func: :query,
    args: [session, query, values],
    args_spec: [T.session(), String.t() | T.query(), T.values()],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode(res)}
        other -> other
      end,
    example_setup: :session_setup,
    doc_example: """
    iex> query = "INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)"
    iex> values = [{:text, "test"}, {:int, 3}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.query(session, query, values)
    iex> # Test Decimal and Varint
    iex> t = "CREATE TABLE IF NOT EXISTS test.types_test (id INT PRIMARY KEY, d DECIMAL, v VARINT);"
    iex> {:ok, _} = Session.query(session, t, [])
    iex> query = "INSERT INTO test.types_test (id, d, v) VALUES (?, ?, ?)"
    iex> values = [{:int, 1}, {:decimal, "1.23"}, {:varint, "12345678901234567890"}]
    iex> {:ok, _} = Session.query(session, query, values)
    iex> {:ok, %QueryResult{rows: [%ExScylla.Types.Row{columns: [id, d, v]}]}} = Session.query(session, "SELECT * FROM test.types_test WHERE id = 1", [])
    iex> {:int, 1} = id
    iex> {:decimal, "1.23"} = d
    iex> {:varint, "12345678901234567890"} = v
    """
  )

  native_f_async(
    func: :query,
    as: :query_raw,
    args: [session, query, values],
    args_spec: [T.session(), String.t() | T.query(), T.values()],
    return_spec: {:ok, QueryResultRaw.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode_raw(res)}
        other -> other
      end
  )

  # # //session::s_query_iter,
  native_f_async(
    func: :query_paged,
    args: [session, query, values, paging_state],
    args_spec: [T.session(), String.t() | T.query(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode(res)}
        other -> other
      end,
    example_setup: :session_setup,
    doc_example: """
    iex> query = "INSERT INTO test.session_doc (a, b, c) VALUES (?, ?, ?)"
    iex> values = [{:text, "test_query_paged"}, {:int, 1}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.query(session, query, values)
    iex> q = Query.new("SELECT * FROM test.session_doc WHERE a = ?;")
    ...>              |> Query.with_page_size(1)
    iex> values = [{:text, "test_query_paged"}]
    iex> {:ok, %QueryResult{paging_state: pgs}} = Session.query_paged(session, q, values, nil)
    iex> true = is_binary(pgs)
    iex> {:ok, %QueryResult{}} = Session.query_paged(session, q, values, pgs)
    """
  )

  native_f_async(
    func: :query_paged,
    as: :query_raw_paged,
    args: [session, query, values, paging_state],
    args_spec: [T.session(), String.t() | T.query(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResultRaw.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    post_process:
      case result do
        {:ok, res} -> {:ok, QueryResult.decode_raw(res)}
        other -> other
      end
  )

  native_f_async(
    func: :refresh_metadata,
    args: [session],
    args_spec: [T.session()],
    return_spec: :ok | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> :ok = Session.refresh_metadata(session)
    """
  )

  native_f_async(
    func: :use_keyspace,
    args: [session, keyspace_name, case_sensitive],
    args_spec: [T.session(), String.t(), boolean()],
    return_spec: :ok | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> case_sensitive = false
    iex> :ok = Session.use_keyspace(session, "another_test_keyspace", case_sensitive)
    """
  )
end
