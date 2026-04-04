defmodule ExScylla.Examples.StreamTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Statement.{Query, Prepared}
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "stream_test")
    :ok = Session.use_keyspace(session, "stream_test", false)

    # Create a table for testing
    {:ok, _} =
      Session.query(
        session,
        "CREATE TABLE IF NOT EXISTS stream_table (a int PRIMARY KEY, b text);",
        []
      )

    # Insert data for streaming tests
    for i <- 1..20 do
      {:ok, _} =
        Session.query(session, "INSERT INTO stream_table (a, b) VALUES (?, ?)",
          int: i,
          text: "val#{i}"
        )
    end

    [session: session]
  end

  test "Session.query_stream with page size", %{session: session} do
    q = Query.new("SELECT * FROM stream_table") |> Query.set_page_size(3)

    # Query using stream
    rows =
      Session.query_stream(session, q, [])
      |> Enum.to_list()

    assert length(rows) == 20
  end

  test "Session.execute_stream", %{session: session} do
    {:ok, ps} = Session.prepare(session, "SELECT * FROM stream_table")
    ps = Prepared.set_page_size(ps, 4)

    rows =
      Session.execute_stream(session, ps, [])
      |> Enum.to_list()

    assert length(rows) == 20
  end

  test "Session.query_stream with error", %{session: session} do
    q = Query.new("SELECT * FROM non_existent_table")

    result = Session.query_stream(session, q, []) |> Enum.to_list()

    assert [{:error, _}] = result
  end

  test "Session.execute_stream with error", %{session: session} do
    {:ok, ps} = Session.prepare(session, "SELECT * FROM stream_table WHERE a = ?")

    result = Session.execute_stream(session, ps, text: "not_an_int") |> Enum.to_list()

    assert [{:error, _}] = result
  end
end
