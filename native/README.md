# native

## Overview
Native (Rust) code for ExScylla. It holds a single Rustler NIF crate that wraps the official `scylla` Rust driver and is loaded by the Elixir module `ExScylla.Native` (`lib/ex_scylla/native.ex`). There are no other native crates.

## Subdirectories & Boundaries
- ex_scylla: [./ex_scylla/README.md](./ex_scylla/README.md) - `cdylib` crate `ex_scylla` (scylla `1.10.0`, rustler `0.39.0`, built with `--cfg scylla_unstable`). It exposes sessions, session builders, queries, prepared statements, batches, execution profiles, and load-balancing policies as opaque `ResourceArc` resources. Retry policies are passed as atoms and speculative-execution policies as structs. Network I/O runs on one shared Tokio runtime and replies to the caller as `{opaque, result}` messages (`async_elixir!`). Query results return raw row bytes that the Elixir layer decodes. Source contract: [./ex_scylla/src/README.md](./ex_scylla/src/README.md).
