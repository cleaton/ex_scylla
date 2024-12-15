alias ExScylla.TestSupport
alias ExScylla.Session

# Start ScyllaDB container
{_container, _node, session} = TestSupport.start_container()
TestSupport.setup_simple_keyspace(session, "load_test")

# Create a simpler test table
table = """
  CREATE TABLE IF NOT EXISTS load_test.simple_table(
    id text,
    value int,
    PRIMARY KEY (id)
  );
"""
{:ok, _} = Session.query(session, table, [])

# Insert test data
insert_query = "INSERT INTO load_test.simple_table(id, value) VALUES (?, ?)"
test_data = Enum.map(1..100, fn i ->
  values = [
    {:text, "key#{i}"},
    {:int, i}
  ]
  {:ok, _} = Session.query(session, insert_query, values)
end)

# Prepare select statement
select_query = "SELECT id, value FROM load_test.simple_table WHERE id = ?"
{:ok, ps} = Session.prepare(session, select_query)

# Store session and prepared statement in ETS for access in benchmark
:ets.new(:exscylla, [:ordered_set, :named_table])
:ets.insert(:exscylla, {:s, session})
:ets.insert(:exscylla, {:ps, ps})

# Run benchmarks with different concurrency levels
Benchee.run(
  %{
    "async_query" => {
      fn {input, keys} ->
        Enum.map(input, fn _ ->
          key = Enum.random(keys)
          Session.async_execute(session, ps, [{:text, key}])
        end)
        |> Enum.map(fn {:ok, _Tag} ->
          receive do
            {{:execute, _}, {:ok, _rows}} -> :ok
            other -> IO.inspect(other)
          end
        end)
      end,
      before_scenario: fn input ->
        keys = Enum.map(1..100, &"key#{&1}")
        {input, keys}
      end
    }
  },
  inputs: %{
    "1 query" => Enum.to_list(1..1),
    "100 queries" => Enum.to_list(1..100),
    "1000 queries" => Enum.to_list(1..1000)
  },
  time: 10,
  memory_time: 2,
  parallel: 4
)
