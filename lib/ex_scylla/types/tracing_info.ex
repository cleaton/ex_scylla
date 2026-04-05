defmodule ExScylla.Types.TracingInfo do
  defstruct [
    :client,
    :command,
    :coordinator,
    :duration,
    :parameters,
    :request,
    :started_at,
    :events
  ]

  @type t :: %__MODULE__{
          client: :inet.ip_address() | nil,
          command: String.t() | nil,
          coordinator: :inet.ip_address() | nil,
          duration: integer() | nil,
          parameters: map() | nil,
          request: String.t() | nil,
          started_at: integer() | nil,
          events: list(ExScylla.Types.TracingEvent.t())
        }
end
