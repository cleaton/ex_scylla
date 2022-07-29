defmodule ExScylla.Statement.Prepared do
  alias ExScylla.Types, as: T
  alias ExScylla.Types.PreparedMetadata
  alias ExScylla.Types.Errors.PartitionKeyError
  alias ExScylla.Types.Errors.SerializeValuesError
  use ExScylla.Macros.Native, [
                          prefix: :ps,
                          docs_rs_path: "/scylla/statement/prepared_statement/struct.PreparedStatement.html",
                          ps_setup: """
                          iex> {:ok, session} = SessionBuilder.new()
                          ...>                  |> SessionBuilder.known_node("127.0.0.1:9042")
                          ...>                  |> SessionBuilder.build()
                          iex> {:ok, ps} = Session.prepare(session, "SELECT * FROM test.s_doc WHERE a = ? AND b = ?;")
                          """
                        ]
  @type msg :: String.t()

  native_f func: :compute_partition_key,
           args: [ps, bound_values],
           args_spec: [T.prepared_statement(), T.values()],
           return_spec: {:ok, binary()} | T.parse_error() | SerializeValuesError.t() | PartitionKeyError.t(),
           example_setup: :ps_setup,
           doc_example: """
           iex> p_key = Prepared.compute_partition_key(ps, [{:text, "hi"}, {:int, 123}])
           iex> true = is_binary(p_key)
           """

  native_f func: :disable_paging,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.disable_paging(ps)
           iex> true = is_reference(ps)
           """

  native_f func: :get_consistency,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: T.consitency() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> nil = Prepared.get_consistency(ps)
           iex> ps = Prepared.set_consistency(ps, :quorum)
           iex> :quorum = Prepared.get_consistency(ps)
           """

  native_f func: :get_id,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: binary(),
           example_setup: :ps_setup,
           doc_example: """
           iex> id = Prepared.get_id(ps)
           iex> true = is_binary(id)
           """

  native_f func: :get_is_idempotent,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: boolean(),
           example_setup: :ps_setup,
           doc_example: """
           iex> false = Prepared.get_is_idempotent(ps)
           """

  native_f func: :get_keyspace_name,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: String.t() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> "test" = Prepared.get_keyspace_name(ps)
           """

  native_f func: :get_page_size,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: pos_integer() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> nil = Prepared.get_page_size(ps)
           iex> ps = Prepared.set_page_size(ps, 10)
           iex> 10 = Prepared.get_page_size(ps)
           """

  native_f func: :get_prepare_tracing_ids,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: list(binary()),
           example_setup: :ps_setup,
           doc_example: """
           iex> [] = Prepared.get_prepare_tracing_ids(ps)
           """

  native_f func: :get_prepared_metadata,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: PreparedMetadata.t(),
           example_setup: :ps_setup,
           doc_example: """
           iex> %PreparedMetadata{} = Prepared.get_prepared_metadata(ps)
           """
  # TODO
  # native_f func: :get_retry_policy,
  #          args: [ps],
  #          args_spec: [T.prepared_statement()],
  #         return_spec: pos_integer() | nil

  native_f func: :get_serial_consistency,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: T.serial_consistency() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> local_serial = Prepared.get_serial_consistency(ps)
           """

  native_f func: :get_statement,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: String.t(),
           example_setup: :ps_setup,
           doc_example: """
           iex> Prepared.get_statement(ps)
           "SELECT * FROM test.s_doc WHERE a = ? AND b = ?;"
           """

  native_f func: :get_table_name,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: String.t() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> Prepared.get_table_name(ps)
           "s_doc"
           """

  native_f func: :get_timestamp,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: T.ts_micros() | nil,
           example_setup: :ps_setup,
           doc_example: """
           iex> nil = Prepared.get_timestamp(ps)
           iex> ts_micro = :os.system_time(:microsecond)
           iex> ps = Prepared.set_timestamp(ps, ts_micro)
           iex> ^ts_micro = Prepared.get_timestamp(ps)
           """

  native_f func: :get_tracing,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: boolean(),
           example_setup: :ps_setup,
           doc_example: """
           iex> false = Prepared.get_tracing(ps)
           """

  native_f func: :is_token_aware,
           args: [ps],
           args_spec: [T.prepared_statement()],
           return_spec: boolean(),
           example_setup: :ps_setup,
           doc_example: """
           iex> true = Prepared.is_token_aware(ps)
           """

  native_f func: :set_consistency,
           args: [ps, consistency],
           args_spec: [T.prepared_statement(), T.consistency()],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_consistency(ps, :one)
           iex> true = is_reference(ps)
           """

  native_f func: :set_is_idempotent,
           args: [ps, is_idempotent],
           args_spec: [T.prepared_statement(), boolean()],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_is_idempotent(ps, true)
           iex> true = is_reference(ps)
           """

  native_f func: :set_page_size,
           args: [ps, page_size],
           args_spec: [T.prepared_statement(), pos_integer()],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_page_size(ps, 10)
           iex> true = is_reference(ps)
           """

  # TODO
  #native_f func: :set_retry_policy,
  #         args: [ps, page_size],
  #         args_spec: [T.prepared_statement(), pos_integer()],
  #         return_spec: T.prepared_statement()

  native_f func: :set_serial_consistency,
           args: [ps, sc],
           args_spec: [T.prepared_statement(), T.serial_consistency() | nil],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_serial_consistency(ps, :local_serial)
           iex> true = is_reference(ps)
           """

  native_f func: :set_timestamp,
           args: [ps, timestamp_micros],
           args_spec: [T.prepared_statement(), T.ts_micros() | nil],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_timestamp(ps, :os.system_time(:microsecond))
           iex> true = is_reference(ps)
           """

  native_f func: :set_tracing,
           args: [ps, should_trace],
           args_spec: [T.prepared_statement(), boolean()],
           return_spec: T.prepared_statement(),
           example_setup: :ps_setup,
           doc_example: """
           iex> ps = Prepared.set_tracing(ps, true)
           iex> true = is_reference(ps)
           """
end
