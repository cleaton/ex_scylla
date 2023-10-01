defmodule SessionBuilderTest do
  use ExUnit.Case, async: true
  alias ExScylla.SessionBuilder
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  doctest SessionBuilder
end
