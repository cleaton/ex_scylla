defmodule SessionBuilderTest do
  use ExUnit.Case, async: true
  alias ExScylla.SessionBuilder
  alias ExScylla.Types.RoundRobinPolicy
  alias ExScylla.Types.SimpleSpeculativeExecutionPolicy
  doctest SessionBuilder
end
