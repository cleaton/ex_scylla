defmodule ExScylla.LoadBalancing.DefaultPolicyBuilder do
  alias ExScylla.Types, as: T

  use ExScylla.Macros.Native,
    prefix: :dpb,
    docs_rs_path: "/scylla/transport/load_balancing/struct.DefaultPolicyBuilder.html"

  native_f(
    func: :build,
    args: [dpb],
    args_spec: [T.default_policy_builder()],
    return_spec: T.load_balancing_policy(),
    doc_example: """
    iex> lbp = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.build()
    iex> true = is_reference(lbp)
    """
  )

  native_f(
    func: :enable_shuffling_replicas,
    args: [epb, enable],
    args_spec: [T.default_policy_builder(), boolean()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.enable_shuffling_replicas(true)
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :latency_awareness,
    args: [epb, latency_awareness_builder],
    args_spec: [T.default_policy_builder(), T.latency_awareness_builder()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> lab = LatencyAwarenessBuilder.new()
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.latency_awareness(lab)
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :new,
    args: [],
    args_spec: [],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :permit_dc_failover,
    args: [epb, permit],
    args_spec: [T.default_policy_builder(), boolean()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.permit_dc_failover(true)
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :prefer_datacenter,
    args: [epb, datacenter_name],
    args_spec: [T.default_policy_builder(), String.t()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.prefer_datacenter("dc1")
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :prefer_datacenter_and_rack,
    args: [epb, datacenter_name, rack_name],
    args_spec: [T.default_policy_builder(), String.t(), String.t()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.prefer_datacenter_and_rack("dc1", "rack1")
    iex> true = is_reference(dpb)
    """
  )

  native_f(
    func: :token_aware,
    args: [epb, is_token_aware],
    args_spec: [T.default_policy_builder(), boolean()],
    return_spec: T.default_policy_builder(),
    doc_example: """
    iex> dpb = DefaultPolicyBuilder.new()
    ...>        |> DefaultPolicyBuilder.token_aware(true)
    iex> true = is_reference(dpb)
    """
  )
end
