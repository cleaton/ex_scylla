defmodule ExScylla.NativeConfig do
  @moduledoc """
  Configuration passed to the native library when it loads.

  The library uses one shared Tokio runtime for all sessions. Its worker thread
  count is read once, when `ExScylla.Native` is loaded:

      config :ex_scylla, tokio_worker_threads: 4

  `nil` (the default) uses Tokio's default of one worker per CPU core. Fewer
  workers lower the per-request cost of waking a worker, which helps when one
  process issues many small requests; more workers help when many connections
  are busy at once. Changing the value requires restarting the VM.
  """

  @doc false
  def load_data do
    case Application.get_env(:ex_scylla, :tokio_worker_threads) do
      n when is_integer(n) and n > 0 -> n
      _ -> nil
    end
  end
end
