defmodule ExScylla.Statement.Batch do
  alias ExScylla.Types, as: T
  use ExScylla.Macros.Native, [
                          prefix: :b,
                          docs_rs_path: "/scylla/statement/batch/struct.Batch.html"
                        ]
  native_f func: :append_statement,
           args: [batch, statement],
           args_spec: [T.batch(), T.prepared_statement() | T.query() | String.t()],
           return_spec: T.batch(),
           doc_example: """
           iex> Batch.new(:unlogged)
           ...>   |> Batch.append_statement("INSERT INTO test (a, b) VALUES (1, 2)")
           ...>   |> is_reference()
           true
           """

  native_f func: :get_execution_profile_handle,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: T.execution_profile_handle() | nil,
           doc_example: """
           iex> eph = ExecutionProfile.builder()
           ...>          |> ExecutionProfileBuilder.build()
           ...>          |> ExecutionProfile.into_handle()
           iex> b = Batch.new(:unlogged)
           ...>   |> Batch.set_execution_profile_handle(eph)
           iex> b |> Batch.get_execution_profile_handle()
           ...>   |> is_reference()
           true
           """

 native_f func: :set_execution_profile_handle,
           args: [batch, profile_handle],
           args_spec: [T.batch(), T.execution_profile_handle() | nil],
           return_spec: T.batch(),
           doc_example: """
           iex> eph = ExecutionProfile.builder()
           ...>          |> ExecutionProfileBuilder.build()
           ...>          |> ExecutionProfile.into_handle()
           iex> b = Batch.new(:unlogged)
           ...>   |> Batch.set_execution_profile_handle(eph)
           iex> b |> Batch.get_execution_profile_handle()
           ...>   |> is_reference()
           true
           """

  native_f func: :get_consistency,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: T.consitency() | nil,
           doc_example: """
           iex> b = Batch.new(:unlogged)
           iex> Batch.get_consistency(b)
           nil
           iex> b = Batch.set_consistency(b, :quorum)
           iex> Batch.get_consistency(b)
           :quorum
           """

  native_f func: :get_is_idempotent,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: boolean(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> Batch.get_is_idempotent(batch)
           false
           """

  # TODO
  # native_f func: :get_retry_policy,
  #          args: [q],
  #          args_spec: [T.query()],
  #         return_spec: pos_integer() | nil

  native_f func: :get_serial_consistency,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: T.serial_consistency() | nil,
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           ...>          |> Batch.set_serial_consistency(:local_serial)
           iex> :local_serial = Batch.get_serial_consistency(batch)
           """


  native_f func: :get_timestamp,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: T.ts_micros() | nil,
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> nil = Batch.get_timestamp(batch)
           iex> ts_micro = :os.system_time(:microsecond)
           iex> batch = Batch.set_timestamp(batch, ts_micro)
           iex> ^ts_micro = Batch.get_timestamp(batch)
           """

  native_f func: :get_tracing,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: boolean(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> false = Batch.get_tracing(batch)
           """

  native_f func: :get_type,
           args: [batch],
           args_spec: [T.batch()],
           return_spec: T.batch_type(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> :unlogged = Batch.get_type(batch)
           """

  native_f func: :new,
           args: [batch_type],
           args_spec: [T.batch_type()],
           return_spec: T.batch(),
           doc_example: """
           iex> Batch.new(:unlogged)
           ...>   |> is_reference()
           true
           """

  native_f func: :new_with_statements,
           args: [batch_type, statements],
           args_spec: [T.batch_type(), list(T.prepared_statement() | T.query() | String.t())],
           return_spec: T.batch(),
           doc_example: """
           iex> Batch.new_with_statements(:unlogged, ["INSERT INTO test (a, b) VALUES (1, 2)"])
           ...>   |> is_reference()
           true
           """

  native_f func: :set_consistency,
           args: [batch, consistency],
           args_spec: [T.batch(), T.consistency()],
           return_spec: T.batch(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> batch = Batch.set_consistency(batch, :quorum)
           iex> true = is_reference(batch)
           """

  native_f func: :set_is_idempotent,
           args: [batch, is_idempotent],
           args_spec: [T.batch(), boolean()],
           return_spec: T.batch(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> batch = Batch.set_is_idempotent(batch, true)
           iex> true = is_reference(batch)
           """

  # TODO
  #native_f prefix: :q,
  #         func: :set_retry_policy,
  #         args: [q, page_size],
  #         args_spec: [T.query(), pos_integer()],
  #         return_spec: T.query()

  native_f func: :set_serial_consistency,
           args: [batch, sc],
           args_spec: [T.batch(), T.serial_consistency() | nil],
           return_spec: T.batch(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> batch = Batch.set_serial_consistency(batch, :serial)
           iex> true = is_reference(batch)
           """

  native_f func: :set_timestamp,
           args: [batch, timestamp_micros],
           args_spec: [T.batch(), T.ts_micros() | nil],
           return_spec: T.batch(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> batch = Batch.set_timestamp(batch, :os.system_time(:microsecond))
           iex> true = is_reference(batch)
           """

  native_f func: :set_tracing,
           args: [batch, should_trace],
           args_spec: [T.batch(), boolean()],
           return_spec: T.batch(),
           doc_example: """
           iex> batch = Batch.new(:unlogged)
           iex> batch = Batch.set_tracing(batch, true)
           iex> true = is_reference(batch)
           """
end
