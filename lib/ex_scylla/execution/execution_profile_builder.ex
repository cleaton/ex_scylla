defmodule ExScylla.Execution.ExecutionProfileBuilder do
  alias ExScylla.Types, as: T
  alias ExScylla.Types.LoadBalancingPolicy

  use ExScylla.Macros.Native,
    prefix: :epb,
    docs_rs_path: "/scylla/transport/execution_profile/struct.ExecutionProfileBuilder.html"

  native_f(
    func: :build,
    args: [epb],
    args_spec: [T.execution_profile_builder()],
    return_spec: T.execution_profile(),
    doc_example: """
    iex> ep = ExecutionProfile.builder()
    ...>        |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :consistency,
    args: [epb, consistency],
    args_spec: [T.execution_profile_builder(), T.consistency()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.consistency(:local_quorum)
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :load_balancing_policy,
    args: [epb, lb_policy],
    args_spec: [T.execution_profile_builder(), LoadBalancingPolicy.t()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :request_timeout,
    args: [epb, timeout_ms],
    args_spec: [T.execution_profile_builder(), pos_integer()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.request_timeout(5_000)
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :retry_policy,
    args: [epb, retry_policy],
    args_spec: [T.execution_profile_builder(), T.retry_policy()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.retry_policy(:default)
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :serial_consistency,
    args: [epb, serial_consistency],
    args_spec: [T.execution_profile_builder(), T.serial_consistency()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.serial_consistency(:local_serial)
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :speculative_execution_policy,
    args: [epb, speculative_execution_policy],
    args_spec: [T.execution_profile_builder(), T.speculative_execution_policy()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> {:ok, ep} = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.speculative_execution_policy(:constant)
    ...>          |> ExecutionProfile.build()
    iex> true = is_reference(ep)
    """
  )
end
