defmodule ExScylla.Execution.ExecutionProfileBuilder do
  alias ExScylla.Types, as: T
  alias ExScylla.Execution.ExecutionProfile

  use ExScylla.Macros.Native,
    prefix: :epb,
    docs_rs_path: "/scylla/transport/execution_profile/struct.ExecutionProfileBuilder.html"

  @spec new :: T.execution_profile_builder()
  def new(), do: ExecutionProfile.builder()

  native_f(
    func: :build,
    args: [epb],
    args_spec: [T.execution_profile_builder()],
    return_spec: T.execution_profile(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>        |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :consistency,
    args: [epb, consistency],
    args_spec: [T.execution_profile_builder(), T.consistency()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.consistency(:local_quorum)
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :load_balancing_policy,
    args: [epb, load_balancing_policy],
    args_spec: [T.execution_profile_builder(), T.load_balancing_policy()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.load_balancing_policy(DefaultPolicy.default())
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :request_timeout,
    args: [epb, timeout_ms],
    args_spec: [T.execution_profile_builder(), pos_integer()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.request_timeout(5_000)
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :retry_policy,
    args: [epb, retry_policy],
    args_spec: [T.execution_profile_builder(), T.retry_policy()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.retry_policy(:default_retry_policy)
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :serial_consistency,
    args: [epb, serial_consistency],
    args_spec: [T.execution_profile_builder(), T.serial_consistency()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.serial_consistency(:local_serial)
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :speculative_execution_policy,
    args: [epb, speculative_execution_policy],
    args_spec: [T.execution_profile_builder(), T.speculative_execution_policy()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.speculative_execution_policy(%PercentileSpeculativeExecutionPolicy{max_retry_count: 3, percentile: 0.95})
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )
end
