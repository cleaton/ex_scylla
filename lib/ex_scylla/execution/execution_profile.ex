defmodule ExScylla.Execution.ExecutionProfile do
  alias ExScylla.Types, as: T

  use ExScylla.Macros.Native, [
                          prefix: :ep,
                          docs_rs_path: "/scylla/transport/execution_profile/struct.ExecutionProfile.html"
                        ]

  native_f(
    func: :builder,
    args: [],
    args_spec: [],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfile.builder()
    ...>          |> ExecutionProfileBuilder.build()
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :into_handle_with_label,
    args: [ep, label],
    args_spec: [T.execution_profile(), String.t()],
    return_spec: T.execution_profile_handle(),
    doc_example: """
    iex> eph = ExecutionProfile.builder()
    ...>          |> ExecutionProfileBuilder.build()
    ...>          |> ExecutionProfile.into_handle_with_label("my_label")
    iex> true = is_reference(eph)
    """
  )

  native_f(
    func: :into_handle,
    args: [ep],
    args_spec: [T.execution_profile()],
    return_spec: T.execution_profile_handle(),
    doc_example: """
    iex> eph = ExecutionProfile.builder()
    ...>          |> ExecutionProfileBuilder.build()
    ...>          |> ExecutionProfile.into_handle()
    iex> true = is_reference(eph)
    """
  )

  native_f(
    func: :to_builder,
    args: [ep],
    args_spec: [T.execution_profile()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfileBuilder.new()
    ...>          |> ExecutionProfileBuilder.build()
    iex> epb = ExecutionProfile.to_builder(ep)
    iex> true = is_reference(ep)
    iex> true = is_reference(epb)
    """
  )
end
