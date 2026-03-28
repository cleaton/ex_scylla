defmodule ExScylla.Types.NodeInfo do
  defstruct [:host_id, :address, :datacenter, :rack]

  @type t :: %__MODULE__{
          host_id: binary(),
          address: {:inet.ip_address(), integer()},
          datacenter: String.t() | nil,
          rack: String.t() | nil
        }
end
