defmodule ExScylla.Types.ClusterState do
  defstruct [:nodes, :keyspaces]

  @type t :: %__MODULE__{
          nodes: list(ExScylla.Types.NodeInfo.t()),
          keyspaces: list(String.t())
        }
end
