defmodule ExScylla.Types.Errors.TranslationError do
  alias ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{Native.scylla_version()}/scylla/transport/errors/enum.TranslationError.html
  """
  @type t :: {:invalid_address_in_rule, msg()}
           | {:no_rule_for_address, msg()}
end
