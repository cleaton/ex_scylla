# NIF for Elixir.ExScylla

## Overview
`ex_scylla` is the Rust crate (`cdylib`, version `0.10.2`, edition 2021) behind the `ExScylla.Native` Elixir module. It builds with Rustler and wraps the official `scylla` Rust driver (`1.8.0`), giving Elixir asynchronous access to ScyllaDB/Cassandra sessions, statements, batches, and execution policies. See [src/](./src/README.md) for the code contract.

## Interface
- `Cargo.toml`
  - `[lib] name = "ex_scylla"`, `crate-type = ["cdylib"]`, `path = "src/lib.rs"`.
  - Key dependencies: `rustler 0.38.0`; `scylla 1.8.0` with features `full-serialization`, `metrics`, `chrono-04`, `bigdecimal-04`, `num-bigint-04`, `unstable-nodejs-rs`; `scylla-cql 1.8.0` with features `full-serialization`, `unstable-nodejs-rs`; `tokio` (`full`); `bytes`, `uuid`, `bigdecimal`, `num-bigint`, `chrono`, `futures`, `tracing`, `tracing-subscriber`.
- `build.rs`: emits `cargo:rustc-cfg=scylla_unstable`.
- `.cargo/config.toml`: adds `--cfg scylla_unstable` to the rustflags. On macOS (`x86_64`/`aarch64-apple-darwin`) it adds `-undefined dynamic_lookup` link args. On `x86_64-unknown-linux-musl` it adds `-crt-static`.
- NIF module name: `Elixir.ExScylla.Native` (loaded from `lib/ex_scylla/native.ex`).
- Subdirectories:
  - [src/](./src/README.md): NIF handlers, resources, type conversions, async runtime.

## Invariants
- The `scylla_unstable` cfg must be set, or driver APIs used by this crate (enabled through `unstable-*` features) will not compile. Both `build.rs` and `.cargo/config.toml` set it, and `mise run lint` also passes it via `RUSTFLAGS`.
- An env `RUSTFLAGS` replaces the rustflags from `.cargo/config.toml`. `lib/ex_scylla/native.ex` sets `RUSTFLAGS="-C instrument-coverage --cfg scylla_unstable"` (and `LLVM_PROFILE_FILE`) for `MIX_ENV=test` builds, and `mise run lint` sets `RUSTFLAGS` for its cargo steps. Those builds therefore lose the per-target flags, including the macOS link args. `scylla_unstable` survives through `build.rs`.
- `scylla` and `scylla-cql` versions must move together (both currently `1.8.0`).
- The crate version tracks the Elixir package version (see the release commits).
- Ignore build artifacts: `target/` (gitignored), `*.profraw` coverage dumps, and `*.long-type-*.txt` compiler diagnostic dumps.

## Side Effects
- Building produces a shared library that Mix/Rustler copies into `_build/<env>/lib/ex_scylla/priv/native`.
- At runtime, loading the library starts a process-global Tokio runtime (see [src/](./src/README.md)).

## Verification
- Run: `mise run lint` (`mix format --check-formatted`, `mix compile --warnings-as-errors`, cargo fmt check, cargo check, clippy with `-D warnings`), `mise run compile`, and `mise run test` (requires Docker).

## Notes

### TODO: reduce serialization overhead for datastructures
