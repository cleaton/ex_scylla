defmodule ExScylla.LoadBalancing.DefaultPolicy do
  use ExScylla.Macros.Native,
  prefix: :dp,
  docs_rs_path: "/scylla/transport/load_balancing/struct.DefaultPolicy.html"

  alias ExScylla.Native
  def default(), do: Native.dp_default()
end
