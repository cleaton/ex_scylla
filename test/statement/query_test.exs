defmodule Statement.QueryTest do
  use ExUnit.Case, async: true
  alias ExScylla.Statement.Query
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  doctest Query
end
