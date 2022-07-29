defmodule ExScylla.Types.UserDefinedType do
  use ExScylla.Macros.Native, [
    docs_rs_path: "/scylla/frame/response/result/enum.CqlValue.html#variant.UserDefinedType"
  ]
  alias ExScylla.Types, as: T

  native_struct type_name: String.t(),
                keyspace: String.t(),
                fields: list({String.t(), T.value() | nil})
end
