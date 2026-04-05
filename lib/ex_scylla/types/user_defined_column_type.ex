defmodule ExScylla.Types.UserDefinedColumnType do
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/frame/response/result/enum.ColumnType.html#variant.UserDefinedType"

  alias ExScylla.Types, as: T

  native_struct(
    type_name: String.t(),
    keyspace: String.t(),
    fields_types: list({String.t(), T.column_type()})
  )
end
