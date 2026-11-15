# NIF for Elixir.ExScylla

## Overview
`ex_scylla` is the Rust crate (`cdylib`, version `0.11.0`, edition 2021) behind the `ExScylla.Native` Elixir module. It builds with Rustler and wraps the official `scylla` Rust driver (`1.10.0`), giving Elixir asynchronous access to ScyllaDB/Cassandra sessions, statements, batches, and execution policies. See [src/](./src/README.md) for the code contract.

## Interface
- `Cargo.toml`
  - `[lib] name = "ex_scylla"`, `crate-type = ["cdylib"]`, `path = "src/lib.rs"`.
  - Key dependencies: `rustler 0.39.0`; `scylla =1.10.0` (exact pin, because `unstable-nodejs-rs` APIs may change in any minor release) with features `full-serialization`, `metrics`, `chrono-04`, `bigdecimal-04`, `num-bigint-04`, `unstable-nodejs-rs`; `tokio` (`full`); `bytes`, `uuid`, `bigdecimal` 0.4, `num-bigint` 0.5 (used for varints through raw bytes; decimals use the num-bigint 0.4 that `bigdecimal` re-exports), `chrono`, `futures`, `tracing`. There is no direct `scylla-cql` dependency.
- `build.rs`: emits `cargo:rustc-cfg=scylla_unstable`.
- `.cargo/config.toml`: adds `--cfg scylla_unstable` to the rustflags. On macOS (`x86_64`/`aarch64-apple-darwin`) it adds `-undefined dynamic_lookup` link args. On `x86_64-unknown-linux-musl` it adds `-crt-static`.
- NIF module name: `Elixir.ExScylla.Native` (loaded from `lib/ex_scylla/native.ex`).
- Subdirectories:
  - [src/](./src/README.md): NIF handlers, resources, type conversions, async runtime.

## Invariants
- The `scylla_unstable` cfg must be set, or driver APIs used by this crate (enabled through `unstable-*` features) will not compile. Both `build.rs` and `.cargo/config.toml` set it, and `mise run lint` also passes it via `RUSTFLAGS`.
- An env `RUSTFLAGS` replaces the rustflags from `.cargo/config.toml`. `lib/ex_scylla/native.ex` sets `RUSTFLAGS="-C instrument-coverage --cfg scylla_unstable"` (and `LLVM_PROFILE_FILE`) for `MIX_ENV=test` builds, and `mise run lint` sets `RUSTFLAGS` for its cargo steps. `native.ex` re-adds the macOS `-undefined dynamic_lookup` link args itself on Darwin. `scylla_unstable` survives through `build.rs`.
- Do not depend on `scylla-cql` directly. Since driver 1.9 it is an internal crate pinned exactly by `scylla` (`=3.0.0` for 1.10.0). Import protocol types from the `scylla::frame` re-exports (backed by `scylla-cql-core`). The minimum Rust version is 1.94 (from `scylla` 1.10.0).
- The crate version tracks the Elixir package version (see the release commits).
- Ignore build artifacts: `target/` (gitignored), `*.profraw` coverage dumps, and `*.long-type-*.txt` compiler diagnostic dumps.

## Side Effects
- Building produces a shared library that Mix/Rustler copies into `_build/<env>/lib/ex_scylla/priv/native`.
- At runtime, loading the library starts a process-global Tokio runtime (see [src/](./src/README.md)).

## Verification
- Run: `mise run lint` (`mix format --check-formatted`, `mix compile --warnings-as-errors`, cargo fmt check, cargo check, clippy with `-D warnings`), `mise run compile`, and `mise run test` (requires Docker).

## Notes

### TODO: reduce serialization overhead for datastructures
