defmodule Statement.PreparedTest do
  use ExUnit.Case, async: true
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  alias ExScylla.SessionBuilder
  alias ExScylla.Statement.Prepared
  alias ExScylla.Session
  alias ExScylla.Types.PreparedMetadata

  setup_all do
    node = Application.get_env(:ex_scylla, :test_node)
    {:ok, session} = SessionBuilder.new()
                    |> SessionBuilder.known_node(node)
                    |> SessionBuilder.build()
    t = """
        CREATE TABLE IF NOT EXISTS test.ps_doc(
          a TEXT,
          b INT,
          c DOUBLE,
          PRIMARY KEY (a, b)
        );
        """
    {:ok, _} = Session.query(session, t, [])
    {:ok, %{session: session}}
  end

  doctest Prepared
end
