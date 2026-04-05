defmodule ExScylla.LoadBalancing.LatencyAwarenessBuilder do
  alias ExScylla.Types, as: T

  use ExScylla.Macros.Native,
    prefix: :lab,
    docs_rs_path: "/scylla/policies/load_balancing/struct.LatencyAwarenessBuilder.html"

  native_f(
    func: :exclusion_threshold,
    args: [lab, exclusion_threshold],
    args_spec: [T.latency_awareness_builder(), number()],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> lab = LatencyAwarenessBuilder.new()
    ...>        |> LatencyAwarenessBuilder.exclusion_threshold(2)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :minimum_measurements,
    args: [lab, minimum_measurements],
    args_spec: [T.latency_awareness_builder(), non_neg_integer()],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> lab = LatencyAwarenessBuilder.new()
    ...>        |> LatencyAwarenessBuilder.minimum_measurements(50)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :new,
    args: [],
    args_spec: [],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> lab = LatencyAwarenessBuilder.new()
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :retry_period,
    args: [lab, retry_period_ms],
    args_spec: [T.latency_awareness_builder(), non_neg_integer()],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> retry_period_ms = 5000
    iex> lab = LatencyAwarenessBuilder.new()
    ...>        |> LatencyAwarenessBuilder.retry_period(retry_period_ms)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :scale,
    args: [lab, scale_ms],
    args_spec: [T.latency_awareness_builder(), non_neg_integer()],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> scale_ms = 5000
    iex> lab = LatencyAwarenessBuilder.new()
    ...>        |> LatencyAwarenessBuilder.scale(scale_ms)
    iex> true = is_reference(lab)
    """
  )

  native_f(
    func: :update_rate,
    args: [lab, update_rate_ms],
    args_spec: [T.latency_awareness_builder(), non_neg_integer()],
    return_spec: T.latency_awareness_builder(),
    doc_example: """
    iex> update_rate_ms = 5000
    iex> lab = LatencyAwarenessBuilder.new()
    ...>        |> LatencyAwarenessBuilder.update_rate(update_rate_ms)
    iex> true = is_reference(lab)
    """
  )
end
