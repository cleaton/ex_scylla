defmodule SessionTest do
  use ExUnit.Case, async: true
  alias ExScylla.Session
  alias ExScylla.SessionBuilder
  alias ExScylla.Statement.Batch
  alias ExScylla.Statement.Prepared
  alias ExScylla.Statement.Query
  alias ExScylla.Types.QueryResult
  alias ExScylla.Types.Token
  doctest Session

  setup_all do
    node = Application.get_env(:ex_scylla, :test_node)

    {:ok, session} =
      SessionBuilder.new()
      |> SessionBuilder.known_node(node)
      |> SessionBuilder.build()

    t = """
    CREATE TABLE IF NOT EXISTS test.session_doc(
      a TEXT,
      b INT,
      c DOUBLE,
      PRIMARY KEY (a, b)
    );
    """

    {:ok, _} = Session.query(session, t, [])
    {:ok, %{session: session}}
  end

  test "get_cluster_state includes nodes, keyspaces, and cluster_name", %{session: session} do
    alias ExScylla.Types.ClusterState
    cluster_state = Session.get_cluster_state(session)
    assert %ClusterState{} = cluster_state
    assert is_list(cluster_state.nodes)
    assert length(cluster_state.nodes) > 0
    assert is_list(cluster_state.keyspaces)
    assert "test" in cluster_state.keyspaces
    assert is_binary(cluster_state.cluster_name) or is_nil(cluster_state.cluster_name)
  end

  test "decimal and varint serialization with 0.4 dependencies", %{session: session} do
    table = """
    CREATE TABLE IF NOT EXISTS test.dec_varint_test (
      id INT PRIMARY KEY,
      dec DECIMAL,
      var VARINT
    );
    """

    {:ok, _} = Session.query(session, table, [])

    insert = "INSERT INTO test.dec_varint_test (id, dec, var) VALUES (?, ?, ?)"

    values = [
      {:int, 42},
      {:decimal, "123456789.987654321"},
      {:varint, "123456789012345678901234567890"}
    ]

    {:ok, _} = Session.query(session, insert, values)

    select = "SELECT id, dec, var FROM test.dec_varint_test WHERE id = ?"

    {:ok, %QueryResult{rows: [%ExScylla.Types.Row{columns: [id, dec, var]}]}} =
      Session.query(session, select, [{:int, 42}])

    assert id == {:int, 42}
    assert dec == {:decimal, "123456789.987654321"}
    assert var == {:varint, "123456789012345678901234567890"}
  end
end
