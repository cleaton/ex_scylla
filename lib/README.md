# lib

## Overview

Elixir source root for the `:ex_scylla` OTP application. It contains no direct
source files; all modules live under `ex_scylla/`, which provides the public API
(`ExScylla.SessionBuilder`, `ExScylla.Session`, statements, execution profiles,
load-balancing builders, and result/error types) over the Rust NIF crate in
`native/ex_scylla/`.

## Subdirectories & Boundaries

- ex_scylla: [./ex_scylla/README.md](./ex_scylla/README.md) - Elixir API layer:
  `ExScylla.Native` NIF stubs, `SessionBuilder` / `Session` (async message-based
  I/O with sync wrappers, decoded and raw result APIs, paging streams), and
  `CqlTypes` value decoding. Children:
  - [./ex_scylla/execution/README.md](./ex_scylla/execution/README.md) - execution profiles and handles.
  - [./ex_scylla/load_balancing/README.md](./ex_scylla/load_balancing/README.md) - default load-balancing policy builders.
  - [./ex_scylla/macros/README.md](./ex_scylla/macros/README.md) - wrapper-generating macros and async dispatch.
  - [./ex_scylla/statement/README.md](./ex_scylla/statement/README.md) - `Query`, `Prepared`, `Batch`.
  - [./ex_scylla/types/README.md](./ex_scylla/types/README.md) - type aliases and structs; [errors](./ex_scylla/types/errors/README.md) typespecs.
