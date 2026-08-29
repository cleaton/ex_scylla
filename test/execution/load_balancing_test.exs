defmodule Execution.LoadBalancingTest do
  use ExUnit.Case, async: true
  alias ExScylla.LoadBalancing.DefaultPolicy
  alias ExScylla.LoadBalancing.DefaultPolicyBuilder
  alias ExScylla.LoadBalancing.LatencyAwarenessBuilder

  doctest DefaultPolicy
  doctest DefaultPolicyBuilder
  doctest LatencyAwarenessBuilder
end
