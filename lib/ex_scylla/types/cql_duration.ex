defmodule ExScylla.Types.CqlDuration do
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/value/struct.CqlDuration.html"

  native_struct(
    months: integer(),
    days: integer(),
    nanoseconds: integer()
  )
end
