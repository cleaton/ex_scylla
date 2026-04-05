defmodule ExScylla.Types.TracingEvent do
  defstruct [:event_id, :activity, :source, :source_elapsed, :thread]

  @type t :: %__MODULE__{
          event_id: binary(),
          activity: String.t() | nil,
          source: :inet.ip_address() | nil,
          source_elapsed: integer() | nil,
          thread: String.t() | nil
        }
end
