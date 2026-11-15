# session_builder (SessionBuilder resource type)

## Overview
Type definition for the `crate::session_builder` NIF module. The NIF handlers (`sb_*`) live in the parent file [`../session_builder.rs`](../session_builder.rs); this directory holds only `types.rs`.

## Interface
- `types.rs`
  - `SessionBuilderResource(pub Mutex<Cell<SessionBuilder>>)`: Rustler resource wrapping `scylla::client::session_builder::SessionBuilder`. Registered in `../lib.rs` `load/2`.

## Invariants
- Unlike statement resources, the builder is **mutated in place**. The `use_builder!` macro in `../session_builder.rs` locks the `Mutex`, `take()`s the builder out of the `Cell`, applies the setter, writes the result back, and returns the **same** `ResourceArc`. Callers holding an older reference observe the change.
- `sb_build` `take()`s the builder out of the `Cell` before building, which leaves a fresh `SessionBuilder::default()` behind. Do not rely on a builder resource keeping its configuration after `sb_build`.
- `Cell::take` requires `SessionBuilder: Default`. `sb_new` creates the builder with `SessionBuilder::new()`, while the resource left behind after `sb_build` holds `SessionBuilder::default()`.
- `sb_build` replies `{:ok, session}` or `{:error, ScyllaNewSessionError}`.
- `sb_tracing_info_fetch_attempts` silently ignores `0` (`NonZeroU32::new` guard) and returns the builder unchanged.

## Side Effects
- None in this file. Session construction (network connection) happens in `sb_build` in `../session_builder.rs`, which runs on the shared Tokio runtime via `async_elixir!`.

## Verification
- Run: `mise run lint` and `mise run test` (requires Docker).
