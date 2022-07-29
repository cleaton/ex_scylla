defmodule SessionTest do
  use ExUnit.Case, async: true
  alias ExScylla.Session
  alias ExScylla.SessionBuilder
  alias ExScylla.Statement.Batch
  alias ExScylla.Statement.Prepared
  alias ExScylla.Statement.Query
  alias ExScylla.Types.BatchResult
  alias ExScylla.Types.QueryResult
  doctest Session

  setup_all do
    {:ok, session} = SessionBuilder.new()
                     |> SessionBuilder.known_node("127.0.0.1:9042")
                     |> SessionBuilder.build()
    t = """
        CREATE TABLE IF NOT EXISTS test.s_doc(
          a TEXT,
          b INT,
          c DOUBLE,
          PRIMARY KEY (a, b)
        );
        """
    {:ok, _} = Session.query(session, t, [])
    :ok
  end

end
