defmodule ExScylla.Types.Errors.BadKeyspaceName do
  alias ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{Native.scylla_version()}/scylla/transport/errors/enum.BadKeyspaceName.html
  """
  @type t :: {:empty, msg()}
           | {:too_long, msg()}
           | {:illegal_character, msg()}
end
