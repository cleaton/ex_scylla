defmodule ExScylla.Types.TableSpec do
  use ExScylla.Macros.Native,
    docs_rs_path: "/scylla/frame/response/result/struct.TableSpec.html"

  native_struct(
    ks_name: String.t(),
    table_name: String.t()
  )
end
