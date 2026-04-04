alias ExScylla.TestSupport
alias ExScylla.Session
alias ExScylla.Statement.Batch

# Start ScyllaDB container
{_container, node, session} = TestSupport.start_container()
[host, port] = String.split(node, ":")

# Create keyspace using TestSupport
TestSupport.setup_simple_keyspace(session, "load_test_erlcass_insert")

# Configure erlcass through application environment
Application.put_env(:erlcass, :keyspace, "load_test_erlcass_insert")
Application.put_env(:erlcass, :cluster_options, [
  {:contact_points, host},
  {:port, String.to_integer(port)},
  {:latency_aware_routing, true},
  {:token_aware_routing, true},
  {:number_threads_io, 4},
  {:queue_size_io, 128000},
  {:core_connections_host, 1},
  {:tcp_nodelay, true},
  {:tcp_keepalive, {true, 60}},
  {:connect_timeout, 5000},
  {:request_timeout, 5000},
  {:retry_policy, {:default, true}}
])

# Start erlcass
Application.ensure_all_started(:erlcass)

keyspace = "load_test_erlcass_insert"

# Create test table
table_query = """
CREATE TABLE IF NOT EXISTS #{keyspace}.test_table(
  col1 ascii,
  col2 bigint,
  col3 blob,
  col4 boolean,
  col5 decimal,
  col6 double,
  col7 float,
  col8 int,
  col9 timestamp,
  col10 uuid,
  col11 varchar,
  col12 varint,
  col13 timeuuid,
  col14 inet,
  PRIMARY KEY (col1)
);
"""
{:ok, _} = Session.query(session, table_query, [])
Session.use_keyspace(session, keyspace, true)

insert_query = "INSERT INTO #{keyspace}.test_table(col1, col2, col3, col4, col5, col6, col7, col8, col9, col10, col11, col12, col13, col14) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
:ok = :erlcass.add_prepare_statement(:add_load_test_record, insert_query)
{:ok, ps} = Session.prepare(session, insert_query)

generate_erlcass_row = fn id ->
  ascii = "hello#{id}"
  bigintpositive = 9223372036854775807
  blob = <<1, 2, 3, 4, 5, 6, 7, 8, 9, 10>>
  booleantrue = true
  decimalpositive = {:erlang.integer_to_binary(1234), 5}
  doublepositive = 5.1235131241221e-6
  floatpositive = 5.12351e-6
  intpositive = 2147483647
  timestamp = 2147483647
  {:ok, uuid} = :erlcass_uuid.gen_random()
  varchar1 = "Юникод"
  varint1 = :erlang.integer_to_binary(1928301970128391280192830198049113123)
  {:ok, timeuuid} = :erlcass_uuid.gen_time()
  inet = "127.0.0.1"

  [
    ascii,
    bigintpositive,
    blob,
    booleantrue,
    decimalpositive,
    doublepositive,
    floatpositive,
    intpositive,
    timestamp,
    uuid,
    varchar1,
    varint1,
    timeuuid,
    inet
  ]
end

generate_exscylla_row = fn id ->
  ascii = "hello#{id}"
  bigintpositive = 9223372036854775807
  blob = <<1, 2, 3, 4, 5, 6, 7, 8, 9, 10>>
  booleantrue = true
  decimalpositive = "0.01234"
  doublepositive = 5.1235131241221e-6
  floatpositive = 5.12351e-6
  intpositive = 2147483647
  timestamp = 2147483647
  {:ok, uuid} = :erlcass_uuid.gen_random()
  varchar1 = "Юникод"
  varint1 = "1928301970128391280192830198049113123"
  {:ok, timeuuid} = :erlcass_uuid.gen_time()
  inet = {127, 0, 0, 1}

  [
    {:ascii, ascii},
    {:big_int, bigintpositive},
    {:blob, blob},
    {:boolean, booleantrue},
    {:decimal, decimalpositive},
    {:double, doublepositive},
    {:float, floatpositive},
    {:int, intpositive},
    {:timestamp, timestamp},
    {:uuid, uuid},
    {:text, varchar1},
    {:varint, varint1},
    {:timeuuid, timeuuid},
    {:inet, inet}
  ]
end

Benchee.run(
  %{
    "erlcass_async" => {
      fn {input, _} ->
        Enum.map(input, fn _ ->
          row = generate_erlcass_row.(System.unique_integer([:positive]))
          {:ok, tag} = :erlcass.async_execute(:add_load_test_record, row)
          tag
        end)
        |> Enum.each(fn tag ->
          receive do
            {:execute_statement_result, ^tag, _result} -> :ok
          end
        end)
      end,
      before_scenario: fn input -> {input, nil} end,
      after_scenario: fn _ -> :ok end
    },
    "exscylla_async" => {
      fn {input, _} ->
        Enum.map(input, fn _ ->
          row = generate_exscylla_row.(System.unique_integer([:positive]))
          {:ok, tag} = Session.async_execute(session, ps, row)
          tag
        end)
        |> Enum.each(fn tag ->
          receive do
            {^tag, _result} -> :ok
          end
        end)
      end,
      before_scenario: fn input -> {input, nil} end,
      after_scenario: fn _ -> :ok end
    },
    "erlcass_batch" => {
      fn {input, _} ->
        stms = Enum.map(input, fn _ ->
          row = generate_erlcass_row.(System.unique_integer([:positive]))
          {:ok, stm} = :erlcass.bind_prepared_statement(:add_load_test_record)
          :ok = :erlcass.bind_prepared_params_by_index(stm, row)
          stm
        end)
        {:ok, tag} = :erlcass.batch_async_execute(1, stms, [])
        receive do
            {:execute_statement_result, ^tag, _result} -> :ok
        end
      end,
      before_scenario: fn input -> {input, nil} end,
      after_scenario: fn _ -> :ok end
    },
    "exscylla_batch" => {
      fn {input, _} ->
        # Use new_with_statements for efficiency
        batch = Batch.new_with_statements(:unlogged, List.duplicate(ps, Enum.count(input)))
        values = Enum.map(input, fn _ -> generate_exscylla_row.(System.unique_integer([:positive])) end)
        
        {:ok, tag} = Session.async_batch(session, batch, values)
        receive do
          {^tag, _result} -> :ok
        end
      end,
      before_scenario: fn input -> {input, nil} end,
      after_scenario: fn _ -> :ok end
    }
  },
  inputs: %{
    "Small (1)" => Enum.to_list(1..1),
    "Medium (100)" => Enum.to_list(1..100),
    "Large (1000)" => Enum.to_list(1..1000)
  },
  time: 10,
  parallel: 1
)
