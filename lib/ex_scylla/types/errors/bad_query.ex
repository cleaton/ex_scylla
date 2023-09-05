defmodule ExScylla.Types.Errors.BadQuery do
  use ExScylla.Macros.Native
  alias ExScylla.Types.Errors.SerializeValuesError
  alias ExScylla.Types.Errors.BadKeyspaceName
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{@scylla_version}/scylla/transport/errors/enum.BadQuery.html
  """
  @type t :: {:serialize_values_error, SerializeValuesError.t()}
           | {:values_too_long_for_key, msg()}
           | {:bad_keyspace_name, BadKeyspaceName.t()}
end
