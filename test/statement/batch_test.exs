defmodule Statement.BatchTest do
  use ExUnit.Case, async: true
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  alias ExScylla.Statement.Batch
  alias ExecutionProfileBuilder
  alias ExecutionProfile
  doctest Batch
end
