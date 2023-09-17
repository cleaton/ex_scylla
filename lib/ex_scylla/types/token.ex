defmodule ExScylla.Types.Token do
    use ExScylla.Macros.Native, [
      docs_rs_path: "/scylla/routing/struct.Token.html"
    ]

  native_struct value: integer()
end
