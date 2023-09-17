defmodule SessionBuilderTest do
  use ExUnit.Case, async: true
  alias ExScylla.SessionBuilder
  alias ExScylla.Types.RoundRobinPolicy
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  doctest SessionBuilder
end
