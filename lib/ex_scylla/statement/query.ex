defmodule ExScylla.Statement.Query do
  alias ExScylla.Types, as: T
  use ExScylla.Macros.Native, [
                          prefix: :q,
                          docs_rs_path: "/scylla/statement/query/struct.Query.html"
                        ]

  native_f func: :get_execution_profile_handle,
            args: [q],
            args_spec: [T.query()],
            return_spec: T.execution_profile_handle() | nil,
            doc_example: """
            iex> eph = ExecutionProfile.builder()
            ...>          |> ExecutionProfileBuilder.build()
            ...>          |> ExecutionProfile.into_handle()
            iex> q = Query.new("SELECT * FROM test;")
            ...>   |> Query.set_execution_profile_handle(eph)
            iex> q |> Query.get_execution_profile_handle()
            ...>   |> is_reference()
            true
            """

  native_f func: :set_execution_profile_handle,
            args: [q, profile_handle],
            args_spec: [T.query(), T.execution_profile_handle() | nil],
            return_spec: T.query(),
            doc_example: """
            iex> eph = ExecutionProfile.builder()
            ...>          |> ExecutionProfileBuilder.build()
            ...>          |> ExecutionProfile.into_handle()
            iex> Query.new("SELECT * FROM test;")
            ...>   |> Query.set_execution_profile_handle(eph)
            ...>   |> is_reference()
            true
            """

  native_f func: :get_request_timeout,
            args: [q],
            args_spec: [T.query()],
            return_spec: T.duration_ms() | nil,
            doc_example: """
            iex> q = Query.new("SELECT * FROM test;")
            ...>   |> Query.set_request_timeout(15000)
            iex> q |> Query.get_request_timeout()
            15000
            """

  native_f func: :set_request_timeout,
            args: [q, timeout_ms],
            args_spec: [T.query(), T.duration_ms() | nil],
            return_spec: T.query(),
            doc_example: """
            iex> q = Query.new("SELECT * FROM test;")
            ...>   |> Query.set_request_timeout(15000)
            iex> q |> Query.get_request_timeout()
            15000
            """

  native_f func: :disable_paging,
           args: [q],
           args_spec: [T.query()],
           return_spec: T.query(),
           doc_example: """
           iex> Query.new("SELECT * FROM test;")
           ...>   |> Query.disable_paging()
           ...>   |> is_reference()
           true
           """

  native_f func: :get_consistency,
           args: [q],
           args_spec: [T.query()],
           return_spec: T.consitency() | nil,
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> Query.get_consistency(q)
           nil
           iex> q = Query.set_consistency(q, :quorum)
           iex> Query.get_consistency(q)
           :quorum
           """

  native_f func: :get_is_idempotent,
           args: [q],
           args_spec: [T.query()],
           return_spec: boolean(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> false = Query.get_is_idempotent(q)
           iex> q = Query.set_is_idempotent(q, true)
           iex> true = Query.get_is_idempotent(q)
           """

  native_f func: :get_page_size,
           args: [q],
           args_spec: [T.query()],
           return_spec: pos_integer() | nil,
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> nil = Query.get_page_size(q)
           iex> q = Query.set_page_size(q, 10)
           iex> 10 = Query.get_page_size(q)
           """

  # TODO
  # native_f func: :get_retry_policy,
  #          args: [q],
  #          args_spec: [T.query()],
  #         return_spec: pos_integer() | nil

  native_f func: :get_serial_consistency,
           args: [q],
           args_spec: [T.query()],
           return_spec: T.serial_consistency() | nil,
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> nil = Query.get_serial_consistency(q)
           iex> q = Query.set_serial_consistency(q, :serial)
           iex> :serial = Query.get_serial_consistency(q)
           """

  native_f func: :get_timestamp,
           args: [q],
           args_spec: [T.query()],
           return_spec: T.ts_micros() | nil,
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> nil = Query.get_timestamp(q)
           iex> ts_micro = :os.system_time(:microsecond)
           iex> q = Query.set_timestamp(q, ts_micro)
           iex> ^ts_micro = Query.get_timestamp(q)
           """

  native_f func: :get_tracing,
           args: [q],
           args_spec: [T.query()],
           return_spec: boolean(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> false = Query.get_tracing(q)
           iex> q = Query.set_tracing(q, true)
           iex> true = Query.get_tracing(q)
           """

  native_f func: :new,
           args: [query_text],
           args_spec: [String.t()],
           return_spec: T.query(),
           doc_example: """
           iex> Query.new("SELECT * FROM test;")
           ...>   |> is_reference()
           true
           """

  native_f func: :set_consistency,
           args: [q, consistency],
           args_spec: [T.query(), T.consistency()],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_consistency(q, :one)
           iex> true = is_reference(q)
           """

  native_f func: :set_is_idempotent,
           args: [q, is_idempotent],
           args_spec: [T.query(), boolean()],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_is_idempotent(q, true)
           iex> true = is_reference(q)
           """

  native_f func: :set_page_size,
           args: [q, page_size],
           args_spec: [T.query(), pos_integer()],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_page_size(q, 10)
           iex> true = is_reference(q)
           """

  # TODO
  #native_f prefix: :q,
  #         func: :set_retry_policy,
  #         args: [q, page_size],
  #         args_spec: [T.query(), pos_integer()],
  #         return_spec: T.query()

  native_f func: :set_serial_consistency,
           args: [q, sc],
           args_spec: [T.query(), T.serial_consistency() | nil],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_serial_consistency(q, :local_serial)
           iex> true = is_reference(q)
           """

  native_f func: :set_timestamp,
           args: [q, timestamp_micros],
           args_spec: [T.query(), T.ts_micros() | nil],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_timestamp(q, :os.system_time(:microsecond))
           iex> true = is_reference(q)
           """

  native_f func: :set_tracing,
           args: [q, should_trace],
           args_spec: [T.query(), boolean()],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.set_tracing(q, true)
           iex> true = is_reference(q)
           """

  native_f func: :with_page_size,
           args: [q, page_size],
           args_spec: [T.query(), pos_integer()],
           return_spec: T.query(),
           doc_example: """
           iex> q = Query.new("SELECT * FROM test;")
           iex> q = Query.with_page_size(q, 10)
           iex> true = is_reference(q)
           """
end
