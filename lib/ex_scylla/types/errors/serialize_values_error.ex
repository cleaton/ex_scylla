defmodule ExScylla.Types.Errors.SerializeValuesError do
  use ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{@scylla_version}/scylla/frame/value/enum.SerializeValuesError.html
  """
  @type t :: {:too_many_values, msg()}
           | {:mixing_named_and_not_named_values, msg()}
           | {:value_too_big, msg()}
           | {:parse_error, msg()}
end
