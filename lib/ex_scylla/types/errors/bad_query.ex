defmodule ExScylla.Types.Errors.BadQuery do
  @moduledoc """
  Represents an error related to a bad query.
  Maps to `scylla::errors::BadQuery`.
  """
  alias ExScylla.Macros.Native
  alias ExScylla.Types.Errors.SerializeValuesError
  alias ExScylla.Types.Errors.BadKeyspaceName
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{Native.scylla_version()}/scylla/errors/enum.BadQuery.html
  """
  @type t ::
          {:serialize_values_error, SerializeValuesError.t()}
          | {:values_too_long_for_key, msg()}
          | {:bad_keyspace_name, BadKeyspaceName.t()}
end
