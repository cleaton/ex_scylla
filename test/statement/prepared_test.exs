defmodule Statement.PreparedTest do
  use ExUnit.Case, async: true
  alias ExScylla.Execution.ExecutionProfile
  alias ExScylla.Execution.ExecutionProfileBuilder
  alias ExScylla.Statement.Prepared
  alias ExScylla.SessionBuilder
  alias ExScylla.Session
  alias ExScylla.Types.PreparedMetadata
  doctest Prepared
end
