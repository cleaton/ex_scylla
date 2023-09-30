defmodule Execution.ExecutionProfileBuilderTest do
  use ExUnit.Case, async: true
  alias ExScylla.Types.PercentileSpeculativeExecutionPolicy
  alias ExScylla.LoadBalancing.DefaultPolicy
  alias ExScylla.Execution.ExecutionProfileBuilder
  alias DefaultPolicy
  doctest ExecutionProfileBuilder
end
