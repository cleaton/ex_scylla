defmodule ExScylla.Types.Errors.NewSessionError do
  use ExScylla.Macros.Native
  alias ExScylla.Types.Errors.DbError
  alias ExScylla.Types.Errors.BadQuery
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{@scylla_version}/scylla/transport/errors/enum.NewSessionError.html
  """
  @type t ::
          {:failed_to_resolve_address, msg()}
          | {:empty_known_nodes_list, msg()}
          | {:db_error, DbError.t()}
          | {:bad_query, BadQuery.t()}
          | {:io_error, msg()}
          | {:protocol_error, msg()}
          | {:invalid_message, msg()}
          | {:timeout_error, msg()}
          | {:too_many_orphaned_stream_ids, msg()}
          | {:unable_to_alloc_stream_id, msg()}
end
