defmodule ExScylla.Execution.Profile do
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
    iex> true = is_reference(ep)
    """
  )

  native_f(
    func: :into_handle,
    args: [ep],
    args_spec: [T.execution_profile()],
    return_spec: T.execution_profile_handle(),
    doc_example: """
    iex> eph = ExecutionProfile.builder()
    ...>          |> ExecutionProfile.build()
    ...>          |> ExecutionProfile.into_handle()
    iex> true = is_reference(ep)
    """
  )
end
