defmodule ExScylla.Examples.ParallelTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "parallel_test")
    :ok = Session.use_keyspace(session, "parallel_test", false)
    
    # Create a table for testing
    {:ok, _} = Session.query(session, "CREATE TABLE IF NOT EXISTS parallel (a int PRIMARY KEY, b text);", [])
    
    [session: session]
  end

  test "concurrent inserts using Task.async_stream", %{session: session} do
    count = 100
    
    # Concurrent inserts
    1..count
    |> Task.async_stream(fn i ->
      {:ok, _} = Session.query(session, "INSERT INTO parallel (a, b) VALUES (?, ?)", [int: i, text: "val#{i}"])
    end, max_concurrency: 10)
    |> Stream.run()
    
    # Verify count
    {:ok, res} = Session.query(session, "SELECT count(*) FROM parallel", [])
    [[big_int: ^count]] = Enum.map(res.rows, fn row -> row.columns end)
  end
end
