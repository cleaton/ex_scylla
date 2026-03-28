defmodule ExScylla.Examples.PagingTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Statement.Query
  alias ExScylla.Types.QueryResult
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "paging_test")
    :ok = Session.use_keyspace(session, "paging_test", false)
    
    # Create a table for testing
    {:ok, _} = Session.query(session, "CREATE TABLE IF NOT EXISTS paging (a int PRIMARY KEY, b text);", [])
    
    # Insert 15 rows
    for i <- 1..15 do
      {:ok, _} = Session.query(session, "INSERT INTO paging (a, b) VALUES (?, ?)", [int: i, text: "val#{i}"])
    end

    [session: session]
  end

  test "manual paging with paging_state", %{session: session} do
    q = Query.new("SELECT * FROM paging") |> Query.with_page_size(6)
    
    # Page 1
    {:ok, %QueryResult{rows: rows1, paging_state: ps1}} = Session.query_paged(session, q, [], nil)
    assert length(rows1) == 6
    assert is_binary(ps1)
    
    # Page 2
    {:ok, %QueryResult{rows: rows2, paging_state: ps2}} = Session.query_paged(session, q, [], ps1)
    assert length(rows2) == 6
    assert is_binary(ps2)
    
    # Page 3 (remaining 3 rows)
    {:ok, %QueryResult{rows: rows3, paging_state: ps3}} = Session.query_paged(session, q, [], ps2)
    assert length(rows3) == 3
    assert ps3 == nil
  end
end
