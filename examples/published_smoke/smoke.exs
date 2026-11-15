# Smoke test for the published ex_scylla package: connect, write and read back.
# Raises (non-zero exit) on any failure. Run with `mix run smoke.exs`.
alias ExScylla.{Session, SessionBuilder}
alias ExScylla.Types.Row

expected = System.get_env("EX_SCYLLA_VERSION", "0.11.0")
node = System.get_env("SCYLLA_NODE", "127.0.0.1:9042")

# 1. The dependency really came from Hex, at the expected version.
{:hex, :ex_scylla, ^expected, _, _, _, "hexpm", _} = Mix.Dep.Lock.read()[:ex_scylla]
^expected = to_string(Application.spec(:ex_scylla, :vsn))
IO.puts("ex_scylla #{expected} installed from Hex and NIF loaded")

# 2. Connect (retry while a freshly started ScyllaDB is still booting).
connect = fn connect, attempts ->
  case SessionBuilder.new() |> SessionBuilder.known_node(node) |> SessionBuilder.build() do
    {:ok, session} ->
      session

    {:error, reason} when attempts > 1 ->
      IO.puts("waiting for ScyllaDB at #{node}: #{inspect(reason)}")
      Process.sleep(2_000)
      connect.(connect, attempts - 1)

    {:error, reason} ->
      raise "could not connect to ScyllaDB at #{node}: #{inspect(reason)}"
  end
end

session = connect.(connect, 60)
IO.puts("connected to #{node}")

# 3. Schema, prepared insert (with a UUID string and a NULL), decoded and raw reads.
{:ok, _} =
  Session.query(
    session,
    """
    CREATE KEYSPACE IF NOT EXISTS published_smoke
    WITH REPLICATION = {'class': 'NetworkTopologyStrategy', 'replication_factor': 1}
    """,
    []
  )

{:ok, _} =
  Session.query(
    session,
    "CREATE TABLE IF NOT EXISTS published_smoke.t (id INT PRIMARY KEY, txt TEXT, u UUID, n INT)",
    []
  )

uuid = "123e4567-e89b-12d3-a456-426614174000"

{:ok, insert} =
  Session.prepare(session, "INSERT INTO published_smoke.t (id, txt, u, n) VALUES (?, ?, ?, ?)")

{:ok, _} = Session.execute(session, insert, [{:int, 1}, {:text, "hello"}, {:uuid, uuid}, nil])

select = "SELECT id, txt, u, n FROM published_smoke.t WHERE id = ?"
{:ok, res} = Session.query(session, select, [{:int, 1}])
[%Row{columns: [{:int, 1}, {:text, "hello"}, {:uuid, ^uuid}, nil]}] = res.rows

{:ok, raw} = Session.query_raw(session, select, [{:int, 1}])
[[1, "hello", ^uuid, nil]] = raw.rows

# 4. Bad input is an error tuple, not a crash.
{:error, {:parse_value, _}} = Session.execute(session, insert, [{:uuid, "not-a-uuid"}])

{:ok, _} = Session.query(session, "DROP KEYSPACE published_smoke", [])
IO.puts("OK: published ex_scylla #{expected} works")
