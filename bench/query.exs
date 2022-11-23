alias ExScylla.SessionBuilder
alias ExScylla.Session
q = "SELECT col1, col2, col3, col4, col5, col6, col7, col8, col9, col10, col11, col12, col13, col14 FROM test_table WHERE col1 = ?"
args = Enum.map(1..100, fn i -> "hello#{i}" end)
keyspace = "load_test_erlcass"
Application.ensure_all_started(:erlcass)
#### SETUP
insert_query = "INSERT INTO #{keyspace}.test_table(col1, col2, col3, col4, col5, col6, col7, col8, col9, col10, col11, col12, col13, col14) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
:ok = :erlcass.add_prepare_statement(:add_load_test_record, insert_query)

for ascii <- args do
  bigintpositive = 9223372036854775807
  blob = <<1,2,3,4,5,6,7,8,9,10>>
  booleantrue = true
  decimalpositive = {:erlang.integer_to_binary(1234), 5}
  doublepositive =  5.1235131241221e-6
  floatpositive = 5.12351e-6
  intpositive = 2147483647
  timestamp = 2147483647
  {:ok, uuid} = :erlcass_uuid.gen_random()
  varchar1 = "Юникод"
  varint1 = :erlang.integer_to_binary(1928301970128391280192830198049113123)
  {:ok, timeuuid} = :erlcass_uuid.gen_time()
  inet = "127.0.0.1"

  :ok = :erlcass.execute(:add_load_test_record, [
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
  ])
end


:ok = :erlcass.add_prepare_statement(:testing_query, {q, 1});
:ets.new(:exscylla, [:ordered_set, :named_table])
{:ok, session} = SessionBuilder.new()
      |> SessionBuilder.known_node("127.0.0.1:9042")
      |> SessionBuilder.default_consistency(:one)
      |> SessionBuilder.use_keyspace("load_test_erlcass", true)
      |> SessionBuilder.tcp_nodelay(true)
      |> SessionBuilder.build()
:ets.insert(:exscylla, {:s, session})
{:ok, ps} = Session.prepare(session, q)
:ets.insert(:exscylla, {:ps, ps})

Benchee.run(
  %{
    "erlcass" => {
      fn {input, _} ->
        Enum.map(input, fn _ -> :erlcass.async_execute(:testing_query, [Enum.random(args)]) end)
        |> Enum.map(fn {ok, _Tag} ->
          receive do
            {:execute_statement_result, _, {:ok, _, _}} ->
              :ok
            other ->
              IO.inspect(other)
          end
        end)
      end,
      before_scenario: fn input ->
        #resource = alter_resource(resource)
        {input, nil}
      end,
      after_scenario: fn _ -> :ok end,
      },
      "exscylla" => {
        fn {input, _} ->
          Enum.map(input, fn _ -> Session.async_execute(session, ps, [{:text, Enum.random(args)}]) end)
          |> Enum.map(fn {ok, _Tag} ->
            receive do
              {{:execute, _}, {:ok, _}} ->
                :ok
              other ->
                IO.inspect(other)
            end
          end)
        end,
        before_scenario: fn input ->
          #resource = alter_resource(resource)
          {input, nil}
        end,
        after_scenario: fn _ -> :ok end,
        },
  },
  inputs: %{
    "Small" => Enum.to_list(1..10),
    #"Medium" => Enum.to_list(1..10_000),
    #"Large" => Enum.to_list(1..100_000)
  },
  time: 10,
  parallel: 20
  #profile_after: {:fprof, []}
)
