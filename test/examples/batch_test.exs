defmodule ExScylla.Examples.BatchTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Statement.Batch
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "batch_test")
    :ok = Session.use_keyspace(session, "batch_test", false)

    [session: session]
  end

  test "unlogged batch", %{session: session} do
    # Unique table for this test
    {:ok, _} =
      Session.query(session, "CREATE TABLE unlogged_batch (a int PRIMARY KEY, b text);", [])

    batch =
      Batch.new(:unlogged)
      |> Batch.append_statement("INSERT INTO unlogged_batch (a, b) VALUES (1, 'one')")
      |> Batch.append_statement("INSERT INTO unlogged_batch (a, b) VALUES (2, 'two')")

    # We must provide a list of value lists, one for each statement in the batch.
    # Since these are simple string statements with values included, we pass empty lists.
    {:ok, _} = Session.batch(session, batch, [[], []])

    {:ok, res} = Session.query(session, "SELECT count(*) FROM unlogged_batch", [])
    [[big_int: 2]] = Enum.map(res.rows, fn row -> row.columns end)
  end

  test "prepared batch", %{session: session} do
    # Unique table for this test
    {:ok, _} =
      Session.query(session, "CREATE TABLE prepared_batch (a int PRIMARY KEY, b text);", [])

    # First, we need to prepare the batch (optional but good for performance if reused)
    # Actually, preparing a batch in Scylla usually means preparing the individual statements
    # but the driver also has prepare_batch which returns a PreparedBatch.

    batch =
      Batch.new(:unlogged)
      |> Batch.append_statement("INSERT INTO prepared_batch (a, b) VALUES (?, ?)")
      |> Batch.append_statement("INSERT INTO prepared_batch (a, b) VALUES (?, ?)")

    {:ok, prepared_batch} = Session.prepare_batch(session, batch)

    values = [
      [int: 3, text: "three"],
      [int: 4, text: "four"]
    ]

    {:ok, _} = Session.batch(session, prepared_batch, values)

    {:ok, res} = Session.query(session, "SELECT count(*) FROM prepared_batch", [])
    [[big_int: 2]] = Enum.map(res.rows, fn row -> row.columns end)
  end
end
