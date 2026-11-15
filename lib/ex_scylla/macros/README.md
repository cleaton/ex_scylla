# ExScylla.Macros

## Overview

Compile-time metaprogramming used by every public wrapper module.
`ExScylla.Macros.Native` turns compact `native_f` / `native_f_async` /
`native_struct` declarations into documented, spec'd functions and structs that
delegate to `ExScylla.Native`, and implements the async reply-dispatch protocol
on the Elixir side.

## Interface

- `ExScylla.Macros.Native` (`native.ex`):
  - `__using__/1` options: `:prefix` (NIF name prefix), `:docs_rs_path`,
    `:docs_rs_crate` (`:scylla` default or `:scylla_cql`), `:moduledoc_extra`
    (text appended to the generated `@moduledoc`); any other keyword is
    stored as a persisted module attribute (used for doctest setup snippets such
    as `:session_setup`). Imports the macros, defines `@type opaque :: any()`, and
    sets `@moduledoc` with the docs.rs link.
  - `native_struct/1`: `defstruct` with `@enforce_keys` for all keys plus `@type t`.
  - `native_f/1` (`:func`, `:args`, `:args_spec`, `:return_spec`, optional
    `:doc_example`, `:example_setup`, `:docs_rs_method`): defines `func/n` calling
    `ExScylla.Native.<prefix>_<func>/n` synchronously.
  - `native_f_async/1` (adds optional `:as`, `:type_map`, `:post_process`): defines
    `async_<as>(args..., opaque \\ {<as>, make_ref()})`, which calls
    `ExScylla.Native.<prefix>_<func>(opaque, args...)` and returns
    `{:ok, opaque}` when the NIF returns `:ok` (else the NIF result), and
    `<as>(args..., timeout_ms \\ 5_000)`, which receives `{^opaque, result}` and
    applies `:post_process` to `var!(result)`.
  - `scylla_version/0`, `scylla_cql_version/0`: driver versions parsed from
    `native/ex_scylla/Cargo.lock` at compile time.

## Invariants

- NIF naming: wrapper `func` with `prefix: :p` always calls `ExScylla.Native.p_func`;
  the `:as` option renames only the Elixir function (for example `query_raw`
  reuses `s_query`).
- Async contract: the NIF receives the opaque term first and must send exactly
  `{opaque, result}` to the calling pid (Rust `async_elixir!`); the sync wrapper
  returns `{:error, :timeout}` after `timeout_ms` (default 5000 ms).
- A late reply arriving after a sync timeout is not consumed and stays in the
  caller's mailbox.
- `type_map` code runs before the NIF call and may rebind argument variables
  (for example `query = to_scylla_query(query)` in `ExScylla.Session`).
- `File.cwd!()` must be the project root at compile time, because the module
  reads `#{File.cwd!()}/native/ex_scylla/Cargo.lock`.

## Side Effects

- Compile time: reads `native/ex_scylla/Cargo.lock`; registers persisted module
  attributes (`:prefix`, `:docs_rs_url`, extra options) on the caller module.
- Runtime (generated sync async-wrappers): blocks the calling process in
  `receive` until the reply or timeout; creates refs via `make_ref/0`.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
