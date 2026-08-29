# Agent Guidelines for ExScylla

This document guides AI coding agents working on the ExScylla codebase to ensure fast bootstrapping, correct patterns, and zero regressions.

## 1. Toolchain & Command Reference

ExScylla uses `mise` for managing language runtimes (Erlang, Elixir, Rust) and task execution.

- **Install runtimes**: `mise install`
- **Install Mix dependencies**: `mise run deps:get`
- **Compile project & NIFs**: `mise run compile`
- **Lint & code checks**: `mise run lint` (mix format check, warnings-as-errors, cargo check, clippy)
- **Format code**: `mise run format` (mix format, cargo fmt)
- **Build (deps + lint + compile)**: `mise run build`
- **Run ExUnit test suite**: `mise run test`
- **Run tests with coverage**: `mise run test:coverage`
- **Run benchmarks**: `mise run bench <name>` (e.g. `mise run bench query`, `mise run bench insert`)
- **Start background ScyllaDB**: `mise run scylla:start`
- **Stop background ScyllaDB**: `mise run scylla:stop`
- **Foreground ScyllaDB container**: `mise run scylla:run`

Do not create or use a `Makefile` or `run_scylla.sh`. Add any new workflow commands to `mise.toml` under `[tasks]`.

## 2. Architecture & Codebase Layout

ExScylla is a thin, high-performance Elixir wrapper around the official Rust `scylla` driver crate via Rustler NIFs.

- `native/ex_scylla/`: Rust NIF implementation.
  - `src/lib.rs`: NIF initialization, resource registration.
  - `src/runtime.rs`: Dedicated multithreaded Tokio runtime initialization.
  - `src/session.rs`, `src/session_builder.rs`: Session and builder NIF handlers.
  - `src/prepared_statement.rs`, `src/query.rs`, `src/batch.rs`: Statement NIF handlers.
  - `src/session/types.rs`, `src/types.rs`: CqlValue <-> Elixir term conversions, structs, enums.
  - `src/execution/`: Execution profiles, load balancing policies, retry & speculative execution policies.
  - `src/errors.rs`: Driver and query error translation.
  - `src/utils.rs`: `async_elixir!` macro, result encoder helpers (`ScyllaResult`, `ToElixir`).
- `lib/ex_scylla/`: Elixir API layer.
  - `lib/ex_scylla/native.ex`: Direct NIF function declarations using `Rustler`.
  - `lib/ex_scylla/macros/native.ex`: Metaprogramming macro generating Elixir wrapper functions with async reply dispatch.
  - `lib/ex_scylla/session.ex`: Public `Session` module.
  - `lib/ex_scylla/session_builder.ex`: Public `SessionBuilder` module.
  - `lib/ex_scylla/statement/`: Statement modules (`Batch`, `Prepared`, `Query`).
  - `lib/ex_scylla/types/`: Structs (`ClusterState`, `QueryResult`, `Row`, `Token`, `Metrics`, errors).
- `test/`: ExUnit test suite using Testcontainers.
- `bench/`: Benchee performance benchmarks comparing ExScylla against `erlcass`.
- `docs/`: Technical OpenWiki documentation.

## 3. Core Invariants & Patterns

### ResourceArc Management
Rust structures (`Session`, `SessionBuilder`, `PreparedStatement`, `Query`, `Batch`, `ExecutionProfile`, `LoadBalancingPolicy`) are allocated on the heap inside `rustler::ResourceArc<T>`. Elixir receives opaque references (`reference()`).

### Immutability & Cloning
Builders and statement setters clone their underlying Rust struct and return a new `ResourceArc`. Avoid mutating statements in hot paths: configure statements once and reuse them.

### Asynchronous Execution (`async_elixir!`)
All network or I/O operations must be asynchronous NIFs:
1. Elixir caller creates a unique reference (or token).
2. The NIF receives the calling process PID and the opaque term.
3. `async_elixir!` captures the PID and term in an `OwnedEnv`, and spawns the future onto the shared Tokio runtime (`runtime::rt()`).
4. Upon completion, `OwnedEnv::send_and_clear` sends `{opaque, result}` directly to the calling Elixir PID.

### Dual Result APIs
ExScylla provides two execution paths:
- **Decoded API** (`query`, `execute`, `batch`): Decodes CQL columns into Elixir tagged tuples (e.g. `{:text, "abc"}`, `{:int, 42}`).
- **Raw API** (`query_raw`, `execute_raw`, `batch_raw`): Returns raw frame binaries wrapped in `ExScylla.Types.QueryResultRaw` to bypass Elixir term construction overhead when doing custom binary parsing or proxying.

## 4. Common Pitfalls & Rules

1. **Docker Requirement**: Tests run against a live ScyllaDB instance managed by Testcontainers (`scylladb/scylla:2026.2`). Docker must be running before starting `mise run test`.
2. **Dual-Layer Synchronization**: When adding a function or type, always update both layers:
   - Add/update the Rust NIF in `native/ex_scylla/src/` and register it in `lib.rs` if it is a resource.
   - Add the NIF signature in `lib/ex_scylla/native.ex`.
   - Add the public Elixir function in `lib/ex_scylla/`.
3. **Type Conversions**: Ensure `CqlValue` variants in `session/types.rs` have bidirectional conversions in `From`/`TryFrom` for `ScyllaValue` and corresponding Elixir types in `lib/ex_scylla/cql_types.ex`.
4. **No Blocking in Schedulers**: Never block inside a NIF synchronously on Tokio futures with `block_on`. Always use `async_elixir!`.
