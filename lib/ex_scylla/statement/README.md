# ExScylla.Statement

## Overview

Public wrappers for CQL statement resources: unprepared `Query`, server-side
`Prepared` statements, and `Batch`. Every function is generated with
`ExScylla.Macros.Native.native_f/1` and forwards synchronously to the
`ExScylla.Native` NIF with the module prefix (`q_`, `ps_`, `b_`). None of these
functions perform network I/O; execution happens through `ExScylla.Session`.

## Interface

- `ExScylla.Statement.Query` (prefix `:q`, Rust `scylla::statement::unprepared::Statement`):
  `new/1`, `with_page_size/2`, getters `get_consistency/1`, `get_serial_consistency/1`,
  `get_is_idempotent/1`, `get_page_size/1`, `get_timestamp/1`, `get_tracing/1`,
  `get_request_timeout/1`, `get_execution_profile_handle/1`; setters
  `set_consistency/2`, `set_serial_consistency/2`, `set_is_idempotent/2`,
  `set_page_size/2`, `set_timestamp/2`, `set_tracing/2`, `set_request_timeout/2`,
  `set_execution_profile_handle/2`.
- `ExScylla.Statement.Prepared` (prefix `:ps`, Rust `PreparedStatement`):
  `compute_partition_key/2` (the Rust NIF returns a bare binary on success, and
  `{:error, PartitionKeyError.t()}` or `{:error, {:parse_value, msg}}` on failure;
  the `@spec` in `prepared.ex` still says `{:ok, binary} | ...`), `get_id/1`,
  `get_statement/1`, `get_keyspace_name/1`, `get_table_name/1`,
  `get_prepared_metadata/1` (`PreparedMetadata.t()`), `get_prepare_tracing_ids/1`,
  `is_confirmed_lwt/1`, `is_token_aware/1`, `get_use_cached_result_metadata/1`,
  `set_use_cached_result_metadata/2`, plus the same consistency / serial
  consistency / idempotence / page size / timestamp / tracing / request timeout /
  execution profile handle getters and setters as `Query`.
- `ExScylla.Statement.Batch` (prefix `:b`, Rust `scylla::statement::batch::Batch`):
  `new/1` (`T.batch_type()`), `new_with_statements/2`, `append_statement/2`
  (accepts `T.prepared_statement() | T.query() | String.t()`), `get_type/1`, plus
  consistency / serial consistency / idempotence / timestamp / tracing / request
  timeout / execution profile handle getters and setters (no page size).

## Invariants

- Statements are immutable from Elixir: Rust setters clone the underlying struct
  (`to_owned()`) and return a new `ResourceArc`, so callers must use the returned
  reference (for example `q = Query.set_tracing(q, true)`). Configure once and
  reuse in hot paths.
- Retry-policy accessors are not exposed: `get_retry_policy`/`set_retry_policy`
  wrappers are commented out, and the `ExScylla.Native.{q,b,ps}_get_retry_policy`
  and `{q,b,ps}_set_retry_policy` NIFs return the bare atom `:not_implemented_yet`
  without cloning anything.
- Every function here must have a matching `ExScylla.Native` stub and Rust NIF in
  `native/ex_scylla/src/query.rs`, `prepared_statement.rs`, or `batch.rs`.

## Side Effects

- Allocates new NIF resources on each setter call. No network, disk, or process
  messaging.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
- Targeted: `mix test test/statement` (doctests for `Query`, `Prepared`, `Batch`)
