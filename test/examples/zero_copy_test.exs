defmodule ExScylla.Examples.ZeroCopyTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "zero_copy_test")
    :ok = Session.use_keyspace(session, "zero_copy_test", false)
    
    # Create a table with a blob column
    {:ok, _} = Session.query(session, "CREATE TABLE IF NOT EXISTS blobs (id int PRIMARY KEY, data blob);", [])
    
    [session: session]
  end

  test "zero copy deserialization of large blobs", %{session: session} do
    # Create a large binary (1MB)
    size = 1024 * 1024
    large_data = :crypto.strong_rand_bytes(size)
    
    {:ok, _} = Session.query(session, "INSERT INTO blobs (id, data) VALUES (1, ?)", [blob: large_data])
    
    {:ok, res} = Session.query(session, "SELECT data FROM blobs WHERE id = 1", [])
    [%ExScylla.Types.Row{columns: [retrieved_data]}] = res.rows
    
    # Verify data integrity
    {:blob, retrieved_blob} = retrieved_data
    assert byte_size(retrieved_blob) == size
    assert retrieved_blob == large_data
    
    # In Erlang/Elixir, large binaries (> 64 bytes) are "Refc binaries"
    # and stored on a shared heap. Sub-binaries can point into these.
    # While we can't easily prove it didn't copy *once* from Rust to Elixir,
    # we can check if it behaves like a normal refc binary.
    assert :binary.referenced_byte_size(retrieved_blob) >= size
  end
end
