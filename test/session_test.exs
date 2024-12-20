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
    {:ok, session} = SessionBuilder.new()
                    |> SessionBuilder.known_node(node)
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
    {:ok, %{session: session}}
  end
end
