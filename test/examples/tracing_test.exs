defmodule ExScylla.Examples.TracingTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Statement.Query
  alias ExScylla.Types.{QueryResult, TracingInfo}
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "tracing_test")
    :ok = Session.use_keyspace(session, "tracing_test", false)

    # Create a table for testing
    {:ok, _} =
      Session.query(
        session,
        "CREATE TABLE IF NOT EXISTS tracing (a int PRIMARY KEY, b text);",
        []
      )

    [session: session]
  end

  test "query tracing and info retrieval", %{session: session} do
    q = Query.new("SELECT * FROM tracing") |> Query.set_tracing(true)
    {:ok, %QueryResult{tracing_id: tid}} = Session.query(session, q, [])
    assert is_binary(tid)

    # Tracing info might take a moment to be persisted by Scylla
    # In a real app we might retry, here we just sleep a bit
    Process.sleep(200)
    {:ok, info} = Session.get_tracing_info(session, tid)
    assert %TracingInfo{} = info
    assert is_list(info.events)
  end
end
