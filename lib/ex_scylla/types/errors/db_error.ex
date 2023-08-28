defmodule ExScylla.Types.Errors.DbError do
  use ExScylla.Macros.Native
  @type msg :: String.t()
  @typedoc """
    For more details, see:
      https://docs.rs/scylla/#{@scylla_version}/scylla/transport/errors/enum.DbError.html
  """
  @type t ::
          {:syntax_error, msg()}
          | {:invalid, msg()}
          | {:already_exists, msg()}
          | {:function_failure, msg()}
          | {:authentication_error, msg()}
          | {:unauthorized, msg()}
          | {:config_error, msg()}
          | {:unavaliable, msg()}
          | {:overloaded, msg()}
          | {:is_bootstrapping, msg()}
          | {:truncate_error, msg()}
          | {:read_timeout, msg()}
          | {:write_timeout, msg()}
          | {:read_failure, msg()}
          | {:write_failure, msg()}
          | {:unprepared, msg()}
          | {:server_error, msg()}
          | {:protocol_error, msg()}
          | {:other, msg()}
end
