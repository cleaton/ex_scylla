# prepared_statement (PreparedStatement resource and metadata types)

## Overview
Type definitions for the `crate::prepared_statement` NIF module. The NIF handlers (`ps_*`) live in the parent file [`../prepared_statement.rs`](../prepared_statement.rs); this directory holds `types.rs`, which wraps the driver's `PreparedStatement` as a Rustler resource and defines the Elixir structs returned by `ps_get_prepared_metadata`.

## Interface
- `types.rs`
  - `PreparedStatementResource(pub PreparedStatement)`: Rustler resource. Registered in `../lib.rs` `load/2`.
  - `impl ToElixir<ResourceArc<PreparedStatementResource>> for PreparedStatement` (via `to_elixir!`).
  - `ScyllaPreparedMetadata` (`NifStruct`, Elixir module `ExScylla.Types.PreparedMetadata`): `col_count: usize`, `pk_indexes: Vec<ScyllaPartitionKeyIndex>`, `col_specs: Vec<ScyllaColumnSpec>`. Built by `impl From<&PreparedStatement>` from `get_variable_col_specs()` and `get_variable_pk_indexes()`.
  - `ScyllaPartitionKeyIndex` (`NifStruct`, Elixir module `ExScylla.Types.PartitionKeyIndex`): `index: u16`, `sequence: u16`, with `From<PartitionKeyIndex>`.

## Invariants
- `NifStruct` field names and `#[module = ...]` values must match the Elixir structs in `lib/ex_scylla/types/`. Renaming a field on one side breaks decoding.
- `PreparedStatementResource` is treated as immutable: every `ps_set_*` handler clones the inner statement (`to_owned()`) and returns a new resource. The exception is `ps_set_retry_policy/1`, which returns `:not_implemented_yet`.
- `ScyllaPreparedMetadata.col_count` is derived as the number of bind-variable column specs (`col_specs.iter().count()`), not read from a driver field.
- `ps_compute_partition_key` raises errors as a `ScyllaPartitionKeyError` term, which the conversion in `../errors.rs` always maps to `ValueTooLong`.
- `ScyllaColumnSpec` is defined in `../session/types.rs`. Metadata conversion depends on its `ToElixir` impl there.

## Side Effects
- None. Pure type and conversion definitions; no I/O.

## Verification
- Run: `mise run lint` and `mise run test` (requires Docker).
