defmodule ExScylla.Examples.BasicTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Statement.Prepared
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "basic_test")
    :ok = Session.use_keyspace(session, "basic_test", false)
    
    # Create a table for testing
    {:ok, _} = Session.query(session, "CREATE TABLE IF NOT EXISTS basic (a int PRIMARY KEY, b text);", [])
    
    [session: session]
  end

  test "basic insert and select", %{session: session} do
    {:ok, _} = Session.query(session, "INSERT INTO basic (a, b) VALUES (?, ?)", [int: 1, text: "foo"])
    {:ok, res} = Session.query(session, "SELECT * FROM basic WHERE a = 1", [])
    assert [%ExScylla.Types.Row{columns: [{:int, 1}, {:text, "foo"}]}] = res.rows
  end

  test "prepared statement execution", %{session: session} do
    {:ok, ps} = Session.prepare(session, "INSERT INTO basic (a, b) VALUES (?, ?)")
    {:ok, _} = Session.execute(session, ps, [int: 2, text: "bar"])
    
    {:ok, ps_select} = Session.prepare(session, "SELECT * FROM basic WHERE a = ?")
    {:ok, res} = Session.execute(session, ps_select, [int: 2])
    assert [%ExScylla.Types.Row{columns: [{:int, 2}, {:text, "bar"}]}] = res.rows
  end

  test "Prepared.get/set_use_cached_result_metadata", %{session: session} do
    {:ok, ps} = Session.prepare(session, "SELECT * FROM basic WHERE a = ?")
    # Default is false in 1.5.0
    assert Prepared.get_use_cached_result_metadata(ps) == false
    ps = Prepared.set_use_cached_result_metadata(ps, true)
    assert Prepared.get_use_cached_result_metadata(ps) == true
  end
end
