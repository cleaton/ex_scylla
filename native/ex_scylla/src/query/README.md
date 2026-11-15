# query (Unprepared Query resource types)

## Overview
Type definitions for the `crate::query` NIF module. The NIF handlers (`q_*`) live in the parent file [`../query.rs`](../query.rs); this directory holds only `types.rs`, which wraps the driver's unprepared statement type (`scylla::statement::unprepared::Statement`, imported as `Query`) as a Rustler resource.

## Interface
- `types.rs`
  - `QueryResource(pub Query)`: Rustler resource (`impl rustler::Resource`, `RefUnwindSafe`). Registered in `../lib.rs` `load/2`. Elixir sees it as an opaque `reference()`.
  - `impl ToElixir<ResourceArc<QueryResource>> for Query` (via `to_elixir!`): `query.ex()` wraps an owned `Query` in a new `ResourceArc`.

## Invariants
- `QueryResource` is treated as immutable: every `q_set_*` and `q_with_page_size` handler in `../query.rs` clones the inner `Query` (`to_owned()`) and returns a new resource through `.ex()`. The exception is `q_set_retry_policy/1`, which returns `:not_implemented_yet`.
- `QueryResource` is also accepted by `ScyllaQuery::QueryResource` and `ScyllaBatchStatement::QueryResource` in `../session/types.rs`. Changing its layout affects those decoders.

## Side Effects
- None. Pure type definitions; no I/O.

## Verification
- Run: `mise run lint` and `mise run test` (requires Docker).
