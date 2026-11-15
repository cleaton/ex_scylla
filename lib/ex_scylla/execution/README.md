# ExScylla.Execution

## Overview

Wrappers for driver execution profiles: a builder, the built profile, and the
handle that sessions and statements reference. All functions are synchronous
`native_f` calls into `ExScylla.Native` (prefixes `:epb`, `:ep`, `:eph`).

## Interface

- `ExScylla.Execution.ExecutionProfileBuilder` (prefix `:epb`): `new/0`
  (delegates to `ExecutionProfile.builder/0`), `build/1`, `consistency/2`,
  `serial_consistency/2`, `load_balancing_policy/2`, `request_timeout/2` (ms, or
  `nil` to clear), `retry_policy/2` (`:default_retry_policy |
  :fallthrough_retry_policy`), `speculative_execution_policy/2`
  (`SimpleSpeculativeExecutionPolicy` or `PercentileSpeculativeExecutionPolicy`
  struct, or `nil` to clear).
- `ExScylla.Execution.ExecutionProfile` (prefix `:ep`): `builder/0`,
  `into_handle/1`, `into_handle_with_label/2`, `to_builder/1`.
- `ExScylla.Execution.ExecutionProfileHandle` (prefix `:eph`):
  `map_to_another_profile/2`, `pointee_to_builder/1`.

## Invariants

- `ExecutionProfileBuilder` setters mutate the builder in place under a `Mutex` and
  return the same reference. `build/1` clones the builder first, so the builder
  stays usable after building.
- Retry policy atoms come from the Rust `NifUnitEnum` `ScyllaRetryPolicy`:
  `:default_retry_policy` and `:fallthrough_retry_policy` (`T.retry_policy`).
- Handles are consumed by `SessionBuilder.default_execution_profile_handle/2` and
  the statement `set_execution_profile_handle/2` setters.
- `map_to_another_profile/2` returns `:ok`. It retargets the handle, so every
  holder of that handle (sessions, statements) sees the new profile.
- Each wrapper must have a matching `ExScylla.Native` stub and Rust NIF under
  `native/ex_scylla/src/execution/`.

## Side Effects

- Allocates NIF resources only. No network or disk I/O.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
- Targeted: `mix test test/execution`
