defmodule Statement.PreparedTest do
  use ExUnit.Case, async: true
  alias ExScylla.Statement.Prepared
  alias ExScylla.SessionBuilder
  alias ExScylla.Session
  alias ExScylla.Types.PreparedMetadata
  doctest Prepared
end
