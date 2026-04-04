alias ExScylla.TestSupport
alias ExScylla.Session
alias ExScylla.Types.QueryResult

# Start ScyllaDB container
{_container, node, session} = TestSupport.start_container()
[host, port] = String.split(node, ":")

TestSupport.setup_simple_keyspace(session, "bench_large")

# Configure erlcass
Application.put_env(:erlcass, :keyspace, "bench_large")
Application.put_env(:erlcass, :cluster_options, [
  {:contact_points, host},
  {:port, String.to_integer(port)},
  {:tcp_nodelay, true}
])
Application.ensure_all_started(:erlcass)

# Create a test table
table_query = """
CREATE TABLE IF NOT EXISTS bench_large.test_table_simple(
  pk int,
  ck int,
  val1 text,
  val2 bigint,
  PRIMARY KEY (pk, ck)
);
"""
{:ok, _} = Session.query(session, table_query, [])

# Insert 1000 rows for pk=1
IO.puts("Inserting 1000 rows...")
insert_query = "INSERT INTO bench_large.test_table_simple(pk, ck, val1, val2) VALUES (?, ?, ?, ?)"
{:ok, ps_insert} = Session.prepare(session, insert_query)

# Batch insert for speed
for i <- 1..1000 do
  values = [
    {:int, 1},
    {:int, i},
    {:text, "value_#{i}"},
    {:big_int, i * 100}
  ]
  {:ok, _} = Session.execute(session, ps_insert, values)
end

# Prepare select statement
select_query = "SELECT pk, ck, val1, val2 FROM bench_large.test_table_simple WHERE pk = ?"
{:ok, ps_select} = Session.prepare(session, select_query)

# erlcass setup
:ok = :erlcass.add_prepare_statement(:testing_query_large, {select_query, 1})

IO.puts("Starting benchmark (1000 rows per query)...")

Benchee.run(
  %{
    "erlcass" => fn ->
      {:ok, _Tag} = :erlcass.async_execute(:testing_query_large, [1])
      receive do
        {:execute_statement_result, _, {:ok, _, _}} -> :ok
        other -> IO.inspect(other)
      end
    end,
    "exscylla (typed)" => fn ->
      {:ok, _res} = Session.execute(session, ps_select, [{:int, 1}])
    end,
    "exscylla_raw (raw)" => fn ->
      {:ok, _res} = Session.execute_raw(session, ps_select, [{:int, 1}])
    end
  },
  time: 10,
  warmup: 2,
  memory_time: 2,
  parallel: 1
)
