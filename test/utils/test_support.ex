defmodule ExScylla.TestSupport do
  alias ExScylla.Session
  alias ExScylla.SessionBuilder

  def start_container do
    case Testcontainers.start_link() do
      {:ok, _} -> :ok
      {:error, {:already_started, _}} -> :ok
    end

    container_config =
      Testcontainers.Container.new("scylladb/scylla:2026.2")
      |> Testcontainers.Container.with_exposed_port(9042)
      |> Testcontainers.Container.with_cmd([
        "--disable-version-check",
        "--skip-wait-for-gossip-to-settle",
        "1",
        "--smp",
        "1",
        "--developer-mode",
        "1"
      ])
      |> Testcontainers.Container.with_waiting_strategy(
        Testcontainers.CommandWaitStrategy.new(
          ["cqlsh", "-e", "SHOW VERSION"],
          # timeout in milliseconds
          120_000,
          # check interval in milliseconds
          100
        )
      )

    {:ok, container} = Testcontainers.start_container(container_config)
    # Prefer container IP on Linux for native Scylla cluster discovery
    host =
      if container.ip_address && container.ip_address != "" do
        container.ip_address
      else
        Testcontainers.get_host() |> String.replace("localhost", "127.0.0.1")
      end

    port =
      if container.ip_address && container.ip_address != "" do
        9042
      else
        Testcontainers.Container.mapped_port(container, 9042)
      end

    node = "#{host}:#{port}"

    IO.puts("ScyllaDB node: #{node}")

    {:ok, session} =
      SessionBuilder.new()
      |> SessionBuilder.known_node(node)
      |> SessionBuilder.build()

    {container, node, session}
  end

  def get_session do
    [{:session, session}] = :ets.lookup(:ex_scylla_test, :session)
    session
  end

  def setup_simple_keyspace(session, keyspace, replication_factor \\ 1) do
    ks = """
      CREATE KEYSPACE IF NOT EXISTS #{keyspace}
      WITH REPLICATION = {
        'class' : 'NetworkTopologyStrategy',
        'replication_factor' : #{replication_factor}
      };
    """

    {:ok, _} = Session.query(session, ks, [])
    :ok
  end
end
