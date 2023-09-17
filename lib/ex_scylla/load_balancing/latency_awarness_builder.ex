defmodule ExScylla.LoadBalancing.LatencyAwarnessBuilder do
  alias ExScylla.Types, as: T

  use ExScylla.Macros.Native,
    prefix: :lab,
    docs_rs_path: "/scylla/transport/load_balancing/struct.LatencyAwarenessBuilder.html"

  native_f(
    func: :exclusion_threshold,
    args: [lab, exclusion_threshold],
    args_spec: [T.latency_awarness_builder(), number()],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> lab = LatencyAwarnessBuilder.new()
    ...>        |> LatencyAwarnessBuilder.lab_exclusion_threshold(2)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :minimum_measurements,
    args: [lab, minimum_measurements],
    args_spec: [T.latency_awarness_builder(), non_neg_integer()],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> lab = LatencyAwarnessBuilder.new()
    ...>        |> LatencyAwarnessBuilder.minimum_measurements(50)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :new,
    args: [],
    args_spec: [],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> lab = LatencyAwarnessBuilder.new()
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :retry_period,
    args: [lab, retry_period_ms],
    args_spec: [T.latency_awarness_builder(), non_neg_integer()],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> retry_period_ms = 5000
    iex> lab = LatencyAwarnessBuilder.new()
    ...>        |> LatencyAwarnessBuilder.retry_period(retry_period_ms)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :scale,
    args: [lab, scale_ms],
    args_spec: [T.latency_awarness_builder(), non_neg_integer()],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> scale_ms = 5000
    iex> lab = LatencyAwarnessBuilder.new()
    ...>        |> LatencyAwarnessBuilder.scale(scale_ms)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :update_rate,
    args: [lab, update_rate_ms],
    args_spec: [T.latency_awarness_builder(), non_neg_integer()],
    return_spec: T.latency_awarness_builder(),
    doc_example: """
    iex> update_rate_ms = 5000
    iex> lab = LatencyAwarnessBuilder.new()
    ...>        |> LatencyAwarnessBuilder.update_rate(update_rate_ms)
    iex> true = is_reference(lab)
    """
  )
end
