defmodule ExScylla.Types.Errors.QueryError do
  alias ExScylla.Types.Errors.DbError
  alias ExScylla.Types.Errors.BadQuery
  alias ExScylla.Types.Errors.TranslationError
  use ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{@scylla_version}/scylla/transport/errors/enum.QueryError.html
  """
  @type t :: {:db_error, DbError.t()}
           | {:bad_query, BadQuery.t()}
           | {:io_error, msg()}
           | {:protocol_error, msg()}
           | {:invalid_message, msg()}
           | {:timeout_error, msg()}
           | {:too_many_oprhaned_stream_ids, msg()}
           | {:unable_to_alloc_stream_id, msg()}
           | {:translation_error, TranslationError.t()}
end
