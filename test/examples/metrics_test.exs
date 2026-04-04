defmodule ExScylla.Examples.MetricsTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.Types.Metrics
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    [session: session]
  end

  test "driver side metrics", %{session: session} do
    # Perform some activity
    {:ok, _} = Session.query(session, "SELECT now() FROM system.local", [])

    metrics = Session.get_metrics(session)
    assert %Metrics{} = metrics
    assert metrics.queries_num > 0
  end
end
