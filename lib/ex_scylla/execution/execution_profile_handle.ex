defmodule ExScylla.Execution.ExecutionProfileHandle do
  alias ExScylla.Types, as: T

  use ExScylla.Macros.Native, [
                          prefix: :eph,
                          docs_rs_path: "/scylla/transport/execution_profile/struct.ExecutionProfileHandle.html"
                        ]

  native_f(
    func: :map_to_another_profile,
    args: [eph, profile],
    args_spec: [T.execution_profile_handle(), T.execution_profile()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> ep = ExecutionProfile.builder() |> ExecutionProfileBuilder.build()
    iex> eph = ExecutionProfile.builder()
    ...>          |> ExecutionProfileBuilder.build()
    ...>          |> ExecutionProfile.into_handle()
    iex> ExecutionProfileHandle.map_to_another_profile(eph, ep)
    """
  )

  native_f(
    func: :pointee_to_builder,
    args: [eph],
    args_spec: [T.execution_profile_handle()],
    return_spec: T.execution_profile_builder(),
    doc_example: """
    iex> eph = ExecutionProfile.builder()
    ...>          |> ExecutionProfileBuilder.build()
    ...>          |> ExecutionProfile.into_handle()
    iex> epb = ExecutionProfileHandle.pointee_to_builder(eph)
    iex> true = is_reference(epb)
    """
  )
end
