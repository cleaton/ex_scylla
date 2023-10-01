defmodule Execution.ExecutionProfileHandleTest do
  use ExUnit.Case, async: true
  alias ExScylla.Execution.ExecutionProfileHandle
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  doctest ExecutionProfileHandle
end
