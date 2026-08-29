defmodule SessionFullCoverageTest do
  use ExUnit.Case, async: false

  alias ExScylla.Session
  alias ExScylla.SessionBuilder
  alias ExScylla.Statement.{Batch, Prepared, Query}
  alias ExScylla.Types.{ClusterState, Metrics, QueryResult, QueryResultRaw, Row, Token}
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "session_full_cov_ks")
    :ok = Session.use_keyspace(session, "session_full_cov_ks", false)

    # Table for stream tests
    {:ok, _} =
      Session.query(
        session,
        """
        CREATE TABLE IF NOT EXISTS test_stream (
          id INT PRIMARY KEY,
          val TEXT
        );
        """,
        []
      )

    for i <- 1..10 do
      {:ok, _} =
        Session.query(session, "INSERT INTO test_stream (id, val) VALUES (?, ?);", [
          {:int, i},
          {:text, "val_#{i}"}
        ])
    end

    # Table for token calculation tests
    {:ok, _} =
      Session.query(
        session,
        """
        CREATE TABLE IF NOT EXISTS test_tokens (
          pk TEXT PRIMARY KEY,
          val INT
        );
        """,
        []
      )

    # Table for batch tests
    {:ok, _} =
      Session.query(
        session,
        """
        CREATE TABLE IF NOT EXISTS test_batch (
          id INT PRIMARY KEY,
          data TEXT
        );
        """,
        []
      )

    # Table for counter batch tests
    {:ok, _} =
      Session.query(
        session,
        """
        CREATE TABLE IF NOT EXISTS test_counters (
          id INT PRIMARY KEY,
          c COUNTER
        );
        """,
        []
      )

    [session: session]
  end

  describe "execute_stream/3 and query_stream/3" do
    test "Session.execute_stream streams multiple pages across result set", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream;")
      ps = Prepared.set_page_size(ps, 2)

      rows =
        Session.execute_stream(session, ps, [])
        |> Enum.to_list()

      assert length(rows) == 10
      assert Enum.all?(rows, fn %Row{columns: [id, val]} -> is_tuple(id) and is_tuple(val) end)
    end

    test "Session.execute_stream returns empty list for queries matching no rows", %{
      session: session
    } do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream WHERE id = ?;")

      rows =
        Session.execute_stream(session, ps, [{:int, 999_999}])
        |> Enum.to_list()

      assert rows == []
    end

    test "Session.execute_stream with parameter values returns matching row", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream WHERE id = ?;")

      rows =
        Session.execute_stream(session, ps, [{:int, 5}])
        |> Enum.to_list()

      assert [%Row{columns: [{:int, 5}, {:text, "val_5"}]}] = rows
    end

    test "Session.execute_stream yields {:error, _} on invalid parameter types", %{
      session: session
    } do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream WHERE id = ?;")

      # Pass a text type to an int column
      result =
        Session.execute_stream(session, ps, [{:text, "not_an_int"}])
        |> Enum.to_list()

      assert [{:error, _reason}] = result
    end

    test "Session.query_stream streams rows with page size and handles error branch", %{
      session: session
    } do
      q = Query.new("SELECT id, val FROM test_stream;") |> Query.set_page_size(3)

      rows =
        Session.query_stream(session, q, [])
        |> Enum.to_list()

      assert length(rows) == 10

      # Query non-existent table yields error tuple and terminates
      err_stream =
        Session.query_stream(session, "SELECT * FROM non_existent_stream_tbl_xyz;", [])
        |> Enum.to_list()

      assert [{:error, _reason}] = err_stream
    end
  end

  describe "prepare_batch/2 and batch execution" do
    test "unlogged batch prepare and execute", %{session: session} do
      batch =
        Batch.new(:unlogged)
        |> Batch.append_statement("INSERT INTO test_batch (id, data) VALUES (?, ?);")
        |> Batch.append_statement("INSERT INTO test_batch (id, data) VALUES (?, ?);")

      {:ok, prepared_batch} = Session.prepare_batch(session, batch)
      assert is_reference(prepared_batch)

      values = [
        [{:int, 1001}, {:text, "data_1001"}],
        [{:int, 1002}, {:text, "data_1002"}]
      ]

      {:ok, %QueryResult{}} = Session.batch(session, prepared_batch, values)

      {:ok, res} =
        Session.query(session, "SELECT id, data FROM test_batch WHERE id IN (1001, 1002);", [])

      assert length(res.rows) == 2
    end

    test "logged batch prepare and execute", %{session: session} do
      batch =
        Batch.new(:logged)
        |> Batch.append_statement("INSERT INTO test_batch (id, data) VALUES (?, ?);")

      {:ok, prepared_batch} = Session.prepare_batch(session, batch)
      assert is_reference(prepared_batch)

      values = [[{:int, 1003}, {:text, "data_1003"}]]
      {:ok, %QueryResult{}} = Session.batch(session, prepared_batch, values)
    end

    test "counter batch prepare and execute", %{session: session} do
      batch =
        Batch.new(:counter)
        |> Batch.append_statement("UPDATE test_counters SET c = c + ? WHERE id = ?;")

      {:ok, prepared_batch} = Session.prepare_batch(session, batch)
      assert is_reference(prepared_batch)

      values = [[{:counter, 10}, {:int, 1}]]
      {:ok, %QueryResult{}} = Session.batch(session, prepared_batch, values)
    end

    test "prepare_batch with invalid statement returns error", %{session: session} do
      batch =
        Batch.new(:unlogged)
        |> Batch.append_statement("MALFORMED SQL STATEMENT ???;")

      assert {:error, _reason} = Session.prepare_batch(session, batch)
    end

    test "batch_raw returns QueryResultRaw", %{session: session} do
      batch =
        Batch.new(:unlogged)
        |> Batch.append_statement("INSERT INTO test_batch (id, data) VALUES (2001, 'raw_data');")

      {:ok, %QueryResultRaw{}} = Session.batch_raw(session, batch, [[]])
    end
  end

  describe "fetch_schema_version/1" do
    test "fetches schema version as binary UUID", %{session: session} do
      {:ok, version} = Session.fetch_schema_version(session)
      assert is_binary(version)
      assert byte_size(version) == 16
    end
  end

  describe "calculate_token/3 and calculate_token_for_partition_key/4" do
    test "calculates deterministic partition token for single column key", %{session: session} do
      token1 =
        Session.calculate_token_for_partition_key(session, "session_full_cov_ks", "test_tokens", [
          {:text, "user_123"}
        ])

      assert %Token{value: t1} = token1
      assert is_integer(t1)

      token2 =
        Session.calculate_token_for_partition_key(session, "session_full_cov_ks", "test_tokens", [
          {:text, "user_123"}
        ])

      assert token1 == token2

      token3 =
        Session.calculate_token_for_partition_key(session, "session_full_cov_ks", "test_tokens", [
          {:text, "user_456"}
        ])

      assert %Token{value: t3} = token3
      assert t1 != t3
    end

    test "calculate_token_for_partition_key returns nil for invalid table or keyspace", %{
      session: session
    } do
      assert Session.calculate_token_for_partition_key(
               session,
               "session_full_cov_ks",
               "non_existent_table_xyz",
               [{:text, "test"}]
             ) == nil

      assert Session.calculate_token_for_partition_key(
               session,
               "non_existent_keyspace_xyz",
               "test_tokens",
               [{:text, "test"}]
             ) == nil
    end

    test "calculate_token matches calculate_token_for_partition_key", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT pk, val FROM test_tokens WHERE pk = ?;")
      token_ps = Session.calculate_token(session, ps, [{:text, "user_123"}])
      assert %Token{value: t_ps} = token_ps

      token_pk =
        Session.calculate_token_for_partition_key(session, "session_full_cov_ks", "test_tokens", [
          {:text, "user_123"}
        ])

      assert token_ps == token_pk
      assert t_ps == token_pk.value

      # With empty or invalid values, calculate_token returns nil
      assert Session.calculate_token(session, ps, []) == nil
    end
  end

  describe "refresh_metadata/1" do
    test "refreshes session metadata successfully", %{session: session} do
      assert :ok = Session.refresh_metadata(session)
    end
  end

  describe "schema agreement" do
    test "await_schema_agreement/1 succeeds", %{session: session} do
      assert :ok = Session.await_schema_agreement(session)
    end

    test "await_timed_schema_agreement/2 succeeds with timeout", %{session: session} do
      assert {:ok, agreed} = Session.await_timed_schema_agreement(session, 15_000)
      assert is_boolean(agreed)
    end

    test "check_schema_agreement/1 returns boolean status", %{session: session} do
      assert {:ok, agreed} = Session.check_schema_agreement(session)
      assert is_boolean(agreed)
    end
  end

  describe "get_default_execution_profile_handle/1" do
    test "returns execution profile handle reference", %{session: session} do
      eph = Session.get_default_execution_profile_handle(session)
      assert is_reference(eph)
    end
  end

  describe "get_keyspace/1 and use_keyspace/3" do
    test "retrieves and switches keyspace" do
      node = Application.get_env(:ex_scylla, :test_node)

      {:ok, fresh_session} =
        SessionBuilder.new()
        |> SessionBuilder.known_node(node)
        |> SessionBuilder.build()

      assert Session.get_keyspace(fresh_session) == nil

      assert :ok = Session.use_keyspace(fresh_session, "session_full_cov_ks", false)
      assert Session.get_keyspace(fresh_session) == "session_full_cov_ks"

      assert :ok = Session.use_keyspace(fresh_session, "test", false)
      assert Session.get_keyspace(fresh_session) == "test"
    end

    test "use_keyspace with non-existent keyspace returns error", %{session: session} do
      assert {:error, _reason} =
               Session.use_keyspace(session, "non_existent_keyspace_99999", false)
    end
  end

  describe "paged and raw queries" do
    test "query_paged and query_raw_paged page navigation", %{session: session} do
      q = Query.new("SELECT id, val FROM test_stream;") |> Query.set_page_size(3)

      {:ok, %QueryResult{rows: p1_rows, paging_state: ps1}} =
        Session.query_paged(session, q, [], nil)

      assert length(p1_rows) == 3
      assert is_binary(ps1)

      {:ok, %QueryResult{rows: p2_rows}} =
        Session.query_paged(session, q, [], ps1)

      assert length(p2_rows) == 3

      {:ok, %QueryResultRaw{paging_state: raw_ps1}} =
        Session.query_raw_paged(session, q, [], nil)

      assert is_binary(raw_ps1)
    end

    test "execute_paged and execute_raw_paged page navigation", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream;")
      ps = Prepared.set_page_size(ps, 3)

      {:ok, %QueryResult{rows: p1_rows, paging_state: ps1}} =
        Session.execute_paged(session, ps, [], nil)

      assert length(p1_rows) == 3
      assert is_binary(ps1)

      {:ok, %QueryResult{rows: p2_rows}} =
        Session.execute_paged(session, ps, [], ps1)

      assert length(p2_rows) == 3

      {:ok, %QueryResultRaw{paging_state: raw_ps1}} =
        Session.execute_raw_paged(session, ps, [], nil)

      assert is_binary(raw_ps1)
    end

    test "query_raw and execute_raw return unpaged QueryResultRaw", %{session: session} do
      {:ok, %QueryResultRaw{}} =
        Session.query_raw(session, "SELECT id, val FROM test_stream;", [])

      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream;")

      {:ok, %QueryResultRaw{}} =
        Session.execute_raw(session, ps, [])
    end
  end

  describe "metrics and cluster state" do
    test "get_metrics returns Metrics struct", %{session: session} do
      metrics = Session.get_metrics(session)
      assert %Metrics{} = metrics
      assert is_integer(metrics.queries_num)
      assert is_integer(metrics.errors_num)
    end

    test "get_cluster_state returns ClusterState struct", %{session: session} do
      cluster_state = Session.get_cluster_state(session)
      assert %ClusterState{} = cluster_state
      assert is_list(cluster_state.nodes)
      assert length(cluster_state.nodes) > 0
    end
  end

  describe "error cases" do
    test "invalid query syntax returns error", %{session: session} do
      assert {:error, _reason} = Session.query(session, "SELECT * FROM ;", [])
    end

    test "query_raw with invalid query syntax returns error", %{session: session} do
      assert {:error, _reason} = Session.query_raw(session, "SELECT * FROM ;", [])
    end

    test "querying non-existent table returns error", %{session: session} do
      assert {:error, _reason} = Session.query(session, "SELECT * FROM non_existent_tbl_xyz;", [])
    end

    test "query_paged and query_raw_paged error returns error", %{session: session} do
      assert {:error, _reason} =
               Session.query_paged(session, "SELECT * FROM non_existent_tbl_xyz;", [], nil)

      assert {:error, _reason} =
               Session.query_raw_paged(session, "SELECT * FROM non_existent_tbl_xyz;", [], nil)
    end

    test "preparing invalid syntax statement returns error", %{session: session} do
      assert {:error, _reason} = Session.prepare(session, "MALFORMED PREPARE STATEMENT")
    end

    test "execute and execute_raw with parameter mismatch return error", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream WHERE id = ?;")
      assert {:error, _reason} = Session.execute(session, ps, [{:text, "not_an_int"}])
      assert {:error, _reason} = Session.execute_raw(session, ps, [{:text, "not_an_int"}])
    end

    test "execute_paged and execute_raw_paged with parameter mismatch return error", %{
      session: session
    } do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream WHERE id = ?;")
      assert {:error, _reason} = Session.execute_paged(session, ps, [{:text, "not_an_int"}], nil)

      assert {:error, _reason} =
               Session.execute_raw_paged(session, ps, [{:text, "not_an_int"}], nil)
    end

    test "batch and batch_raw with invalid query execution return error", %{session: session} do
      batch =
        Batch.new(:unlogged)
        |> Batch.append_statement("INSERT INTO non_existent_ks_xyz.tbl (a) VALUES (?);")

      values = [[{:int, 1}]]
      assert {:error, _reason} = Session.batch(session, batch, values)
      assert {:error, _reason} = Session.batch_raw(session, batch, values)
    end

    test "execute_stream and query_stream consume all rows", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT id, val FROM test_stream;")
      rows = Session.execute_stream(session, ps, []) |> Enum.to_list()
      assert length(rows) == 10

      q_rows =
        Session.query_stream(session, "SELECT id, val FROM test_stream;", []) |> Enum.to_list()

      assert length(q_rows) == 10
    end

    test "QueryResult decode and decode_raw with null column values and non-binary rows", %{
      session: session
    } do
      {:ok, _} =
        Session.query(
          session,
          "CREATE TABLE IF NOT EXISTS test_nulls (id INT PRIMARY KEY, val TEXT);",
          []
        )

      {:ok, _} = Session.query(session, "INSERT INTO test_nulls (id) VALUES (888);", [])

      {:ok, res} = Session.query(session, "SELECT id, val FROM test_nulls WHERE id = 888;", [])
      assert [%Row{columns: [{:int, 888}, nil]}] = res.rows

      {:ok, raw_res} =
        Session.query_raw(session, "SELECT id, val FROM test_nulls WHERE id = 888;", [])

      assert [[888, nil]] = raw_res.rows

      # Non-binary rows passthrough branch
      empty_res = %QueryResult{
        rows: nil,
        rows_count: nil,
        column_types: [],
        warnings: [],
        tracing_id: nil,
        paging_state: nil,
        serialized_size: 0
      }

      assert %QueryResult{rows: nil} = QueryResult.decode(empty_res)
      assert %QueryResultRaw{rows: nil} = QueryResult.decode_raw(empty_res)
    end
  end
end
