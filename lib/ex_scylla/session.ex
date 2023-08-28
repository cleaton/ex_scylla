defmodule ExScylla.Session do
  alias ExScylla.Types, as: T
  alias ExScylla.Types.BatchResult
  alias ExScylla.Types.QueryResult
  alias ExScylla.Types.Errors.QueryError

  use ExScylla.Macros.Native,
    prefix: :s,
    docs_rs_path: "/scylla/transport/session/struct.Session.html",
    session_setup: """
    iex> {:ok, session} = SessionBuilder.new()
    ...>                  |> SessionBuilder.known_node("127.0.0.1:9042")
    ...>                  |> SessionBuilder.build()
    """

  defp to_scylla_query(q) when is_binary(q), do: {:string, q}
  defp to_scylla_query(q) when is_reference(q), do: {:query_resource, q}

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
    return_spec: :ok | {:error, QueryError.t()},
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
    return_spec: {:ok, BatchResult.t()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> batch = Batch.new(:unlogged)
    ...>   |> Batch.append_statement("INSERT INTO test.s_doc (a, b, c) VALUES (?, ?, ?)")
    iex> values = [
    ...>   [{:text, "test"}, {:int, 2}, {:double, 1.0}]
    ...> ]
    iex> {:ok, %BatchResult{}} = Session.batch(session, batch, values)
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
    func: :execute,
    args: [session, prepared, values],
    args_spec: [T.session(), T.prepared_statement(), T.values()],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "INSERT INTO test.s_doc (a, b, c) VALUES (?, ?, ?)")
    iex> values = [{:text, "test"}, {:int, 2}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.execute(session, ps, values)
    """
  )

  native_f_async(
    func: :execute_paged,
    args: [session, prepared, values, paging_state],
    args_spec: [T.session(), T.prepared_statement(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.s_doc WHERE a = ?;")
    iex> ps = Prepared.set_page_size(ps, 1)
    iex> values = [{:text, "test"}]
    iex> {:ok, %QueryResult{paging_state: pgs}} = Session.execute_paged(session, ps, values, nil)
    iex> true = is_binary(pgs)
    iex> {:ok, %QueryResult{}} = Session.execute_paged(session, ps, values, pgs)
    """
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

  # # //session::s_get_cluster_data,
  # # //session::s_get_metrics,
  # # //session::s_get_tracing_info,
  # # //session::s_get_tracing_info_custom,
  native_f_async(
    func: :prepare,
    args: [session, query],
    args_spec: [T.session(), String.t() | T.query()],
    return_spec: {:ok, T.prepared_statement()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    example_setup: :session_setup,
    doc_example: """
    iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.s_doc WHERE a = ?;")
    iex> true = is_reference(ps)
    """
  )

  native_f_async(
    func: :query,
    args: [session, query, values],
    args_spec: [T.session(), String.t() | T.query(), T.values()],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    example_setup: :session_setup,
    doc_example: """
    iex> query = "INSERT INTO test.s_doc (a, b, c) VALUES (?, ?, ?)"
    iex> values = [{:text, "test"}, {:int, 3}, {:double, 1.0}]
    iex> {:ok, %QueryResult{}} = Session.query(session, query, values)
    """
  )

  # # //session::s_query_iter,
  native_f_async(
    func: :query_paged,
    args: [session, query, values, paging_state],
    args_spec: [T.session(), String.t() | T.query(), T.values(), T.paging_state() | nil],
    return_spec: {:ok, QueryResult.t()} | {:error, QueryError.t()},
    type_map: query = to_scylla_query(query),
    example_setup: :session_setup,
    doc_example: """
    iex> q = Query.new("SELECT * FROM test.s_doc WHERE a = ?;")
    ...>              |> Query.with_page_size(1)
    iex> values = [{:text, "test"}]
    iex> {:ok, %QueryResult{paging_state: pgs}} = Session.query_paged(session, q, values, nil)
    iex> true = is_binary(pgs)
    iex> {:ok, %QueryResult{}} = Session.query_paged(session, q, values, pgs)
    """
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
