defmodule ExScylla.Types.Errors.SerializeValuesError do
  @moduledoc """
  Represents an error that can occur when serializing values.
  Maps to `scylla::serialize::SerializationError`.
  """
  alias ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{Native.scylla_version()}/scylla/serialize/struct.SerializationError.html
  """
  @type t ::
          {:too_many_values, msg()}
          | {:mixing_named_and_not_named_values, msg()}
          | {:value_too_big, msg()}
          | {:parse_error, msg()}
end
