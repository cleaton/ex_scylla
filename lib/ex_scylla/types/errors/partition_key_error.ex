defmodule ExScylla.Types.Errors.PartitionKeyError do
  alias ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{Native.scylla_version()}/scylla/statement/prepared_statement/enum.PartitionKeyError.html
  """
  @type t :: {:no_pk_index_value, msg()}
           | {:value_too_long, msg()}
end
