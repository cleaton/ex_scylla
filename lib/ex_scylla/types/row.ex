defmodule ExScylla.Types.Row do
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/value/struct.Row.html"

  native_struct(columns: list(term()))
end
