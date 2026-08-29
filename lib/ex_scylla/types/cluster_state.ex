defmodule ExScylla.Types.ClusterState do
  defstruct [:nodes, :keyspaces, :cluster_name]

  @type t :: %__MODULE__{
          nodes: list(ExScylla.Types.NodeInfo.t()),
          keyspaces: list(String.t()),
          cluster_name: String.t() | nil
        }
end
