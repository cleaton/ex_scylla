# Dependency and Toolchain Upgrade (2026-11)

*   **Author:** Jesper Lundgren
*   **Status:** Implemented, uncommitted (written autonomously; awaiting author review)
*   **Date:** 2026-11-15

--------------------------------------------------------------------------------

## Overview

This design moves the toolchain that builds ExScylla (Erlang/OTP, Elixir, Rust)
and every direct dependency (Rust crates and Mix packages) to their latest major
versions. The Rust driver upgrade (`scylla` 1.8.0 to 1.10.0) is the only change
that touches source code. Everything else is version pins, lock files, and
documentation.

## The Problem

The project pins Erlang 28.4.1, Elixir 1.19.5 and Rust 1.97.1 in `mise.toml`,
and `scylla` 1.8.0, `scylla-cql` 1.8.0 and `rustler` 0.38.0 in
`native/ex_scylla/Cargo.toml`. Newer major versions now exist for each of them.
Staying behind has these costs:

*   `scylla` 1.10.0 raised its minimum Rust version to 1.94 and made
    `scylla-cql` an internal crate (2.0 in driver 1.9, 3.0 in driver 1.10). The
    old `scylla-cql = "1.8.0"` pin cannot move together with `scylla` any
    more, which breaks a rule written in our own contracts.
*   `mix.exs` declares `elixir: "~> 1.13"`, but the locked `rustler` 0.38.0
    requires `~> 1.15`. Users on Elixir 1.13 or 1.14 cannot build the package
    today, and the declared requirement is wrong.
*   The test and benchmark ScyllaDB image (`scylladb/scylla:2026.2`) is one
    release behind the newest stable image (`2026.3`).

The proposed solution is to bump each pin to the newest release, remove the
direct `scylla-cql` dependency, import the protocol types from the
`scylla::frame` re-exports instead, and update every document that names a
version.

## Goals

*   `mise.toml` pins Erlang 29.1.2, Elixir 1.20.5-otp-29 and Rust 1.100.0 (the
    newest stable releases on 2026-11-15) and `cargo:grcov` 0.10.8.
*   `native/ex_scylla/Cargo.toml` uses `scylla` 1.10.0 and `rustler` 0.39.0, and
    no longer depends on `scylla-cql` directly.
*   Every other crate and Mix package resolves to its newest version that the
    rest of the dependency graph allows, recorded in `Cargo.lock` and `mix.lock`.
*   `mix.exs` declares an Elixir requirement that the locked dependencies
    satisfy.
*   `mise run lint` and `mise run test` pass on the new toolchain with no new
    warnings.
*   Contracts and docs that name versions match the new versions.

## Non-Goals

*   Fixing the code bugs found by the 2026-11-15 drift review (UDT
    `field_types` vs `fields_types`, the `:fall_through_retry_policy` spec
    typo, error typespec typos, error variants that collapse to `:io_error`).
    They are unrelated to versions and belong in a separate change, so the
    upgrade diff stays about versions.
*   Moving the crate from Rust edition 2021 to 2024. That is a language change,
    not a dependency upgrade.
*   Adopting new driver features (`BoundStatement`, `try_get_token_endpoints`,
    the `serde` feature, `Display` for column types).
*   Forcing transitive Mix packages past what their parents allow (`hackney`
    1.x to 4.x, `idna`, `metrics`, `certifi`, `uniq`, `parse_trans`). Those
    are held back by `testcontainers` and `tesla` and are test-only.
*   Releasing a new Hex version. The version bump and `CHANGELOG.md` entry
    happen when the author decides to release.

## Open Questions

*   **Rustler crate vs Hex package skew**: the `rustler` crate is at 0.39.0, but
    the newest `rustler` Hex package is 0.38.0. The Hex package only drives
    `cargo build` and loads the library. Nothing found in the changelog says the
    two must share a minor version, but Rustler has always released them in
    lockstep. If tests pass with the skew, this design keeps crate 0.39.0. The
    author should confirm this or pin the crate back to 0.38.0 until Hex 0.39
    ships.
*   **Lowest supported Elixir/OTP**: this design raises `mix.exs` to
    `~> 1.15` because `rustler` already requires it. It does not test 1.15
    itself. Should CI test the lowest supported version as well as the newest?
*   **`num-bigint` 0.5**: resolved. Varints are converted through raw bytes,
    so `num-bigint` 0.5 works. Decimals use the 0.4 that `bigdecimal` re-exports.

## Critical User Journeys (CUJs)

### CUJ 1: Contributor builds and tests on the new toolchain

*   **Given**: a clean checkout with Docker running.
*   **When**: the contributor runs `mise install`, `mise run build` and
    `mise run test`.
*   **Then**: mise installs Erlang 29.1.2, Elixir 1.20.5-otp-29 and Rust
    1.100.0. Lint passes with `--warnings-as-errors` and clippy `-D warnings`.
    The ExUnit suite passes against `scylladb/scylla:2026.3`.

### CUJ 2: Library user depends on ExScylla

*   **Given**: an application on Elixir 1.15 or newer that lists
    `{:ex_scylla, ...}` as a dependency.
*   **When**: it runs `mix deps.get` and `mix compile`.
*   **Then**: the NIF compiles against `scylla` 1.10.0, and the public Elixir API
    behaves as before. No public function, type or return shape changes.

## Technical Plan (Architecture)

The layering does not change. Only versions and three import paths in the
crate change.

```text
mise.toml (toolchain pins)
   |-- erlang 28.4.1  -> 29.1.2
   |-- elixir 1.19.5  -> 1.20.5-otp-29
   |-- rust   1.97.1  -> 1.100.0
   v
mix.exs ---------------------------> native/ex_scylla/Cargo.toml
  rustler (hex) 0.38.0 (newest)        rustler 0.38.0 -> 0.39.0
  elixir req ~> 1.13 -> ~> 1.15        scylla  1.8.0  -> 1.10.0
  ex_doc / lcov_ex / erlcass bumps     scylla-cql 1.8.0 -> removed
                                         (types now via scylla::frame::*,
                                          which re-exports scylla-cql-core)
```

### Rust driver change

`scylla` 1.10.0 pins `scylla-cql = "=3.0.0"` and adds `scylla-cql-core 1.10.0`.
The types this crate imports from `scylla_cql` are re-exported publicly by
`scylla`. Measured in a scratch build:

| Current import (`scylla_cql::...`) | New import (`scylla::...`) | File |
| --- | --- | --- |
| `frame::Compression` | `frame::Compression` | `src/types.rs` |
| `frame::response::result::{CollectionType, ColumnSpec, ColumnType, NativeType}` | same path under `scylla::frame::response::result` | `src/session/types.rs` |
| `frame::response::result::UserDefinedType` | same path under `scylla::frame::response::result` | `src/session/types.rs` |

With only those three path changes, a scratch copy of the crate passed
`cargo check` and `cargo clippy --all-targets -- -D warnings` on Rust 1.97.1.
The `unstable-nodejs-rs` feature still exists on `scylla` 1.10.0. `scylla-cql`
3.0.0 no longer has it, which is a second reason to drop the direct dependency.

Driver behavior changes to watch in tests:

*   Deserializing an empty `varint` now fails. ExScylla does not deserialize
    rows in Rust (rows are decoded in Elixir from raw bytes), so this should
    not affect the result path. Bound `{:varint, ""}` values are converted
    Elixir-to-Rust and already become `CqlValue::Empty` on parse failure.
*   `ClusterState::get_token_endpoints` is deprecated. ExScylla does not call
    it.

### Rustler 0.39

The 0.39.0 changelog lists no breaking changes for crate users. It removes
`rustler::wrapper` and changes `ERL_NIF_TERM` into a newtype, and this crate uses
neither. The scratch build confirmed it compiles unchanged. NIF version 2.15
(the default feature) loads on OTP 29.

### Elixir side

Elixir 1.20 adds more type-system warnings. Because `mise run lint` uses
`--warnings-as-errors`, any new warning in `lib/` must be fixed by changing
code, not by suppressing it. The compile step decides whether this is needed.

### Docs-link helper

`ExScylla.Macros.Native` reads the `scylla-cql` version from `Cargo.lock` to
build docs.rs links for `PreparedMetadata`. `scylla-cql` is still in the lock
file as a transitive dependency, so the parser keeps working. The link will
point at `scylla-cql` 3.0.0 docs.

## Alternatives Considered

### Alternative A: Keep a direct `scylla-cql = "3.0.0"` dependency

*   **Pros**: no import changes.
*   **Cons**: the driver release notes call `scylla-cql` internal and advise
    removing direct dependencies. Its `=3.0.0` pin inside `scylla` means any
    mismatch fails resolution. It no longer has `unstable-nodejs-rs`.
*   **Reason for Rejection**: Rejected by author: the public re-exports cover
    every type we use, so the extra dependency adds risk and no value.

### Alternative B: Upgrade only to the newest minor inside current majors

*   **Pros**: smallest diff. No toolchain changes for contributors.
*   **Cons**: does not meet the request ("all dependencies to latest major
    version"). Leaves the `scylla-cql` pin rule broken.
*   **Reason for Rejection**: Rejected by author: out of line with the stated
    goal.

### Alternative C: Pin `rustler` crate to 0.38.0 to match the Hex package

*   **Pros**: keeps the long-standing lockstep between the crate and the Hex
    package.
*   **Cons**: not the newest major. The 0.39 changes are internal to the crate.
*   **Reason for Rejection**: Kept as the fallback. It applies if the test suite
    shows any load-time or runtime problem with the skew (see Open Questions).

--------------------------------------------------------------------------------

## Non-Functional Requirements

*   **Rollout & Rollback**: all changes are in pins and lock files plus three
    import lines. Rolling back means reverting the change set. There is no data
    migration.
*   **Compatibility**: raising the Elixir requirement to `~> 1.15` documents a
    restriction that already exists (from `rustler`). It is not a new break
    for users. The Rust MSRV becomes 1.94 (from `scylla` 1.10.0), which only
    matters to people building with a toolchain older than the one mise pins.
*   **Performance**: no expected change. The `insert`, `query` and
    `large_result` benchmarks can confirm it if needed.

--------------------------------------------------------------------------------

### Additional Resources

*   `2026-11-15_DRIFT_REPORT.md` (code bugs deliberately left out of scope)
*   scylla-rust-driver release notes for 1.9.0 and 1.10.0
*   rustler `CHANGELOG.md` entries for 0.38.0 and 0.39.0

### Future Improvements

*   **Edition 2024**: move the crate once the upgrade has settled.
*   **Docs links**: point `PreparedMetadata` docs at the `scylla` crate
    re-export instead of internal `scylla-cql`.
*   **CI matrix**: test the lowest supported Elixir/OTP too.

--------------------------------------------------------------------------------

## Detailed Implementation Plan

### Target Files to Create/Modify

#### 1. `mise.toml` (Modify)

*   **Type / Scope**: toolchain and task configuration.
*   **Proposed Changes**: `rust = "1.100.0"`, `erlang = "29.1.2"`,
    `elixir = "1.20.5-otp-29"`, `"cargo:grcov" = "0.10.8"`. Change the
    `scylla:run` and `scylla:start` image to `scylladb/scylla:2026.3`.

#### 2. `native/ex_scylla/Cargo.toml` (Modify)

*   **Type / Scope**: Rust build manifest.
*   **Proposed Changes**: `rustler = "0.39.0"`, `scylla = "1.10.0"` (same
    features). Remove the `scylla-cql` line. Raise the floor requirements of
    the other crates to their current newest versions (`bytes 1.12`,
    `tokio 1.54`, `futures 0.3.34`, `uuid 1.29`, `bigdecimal 0.4.11`,
    `chrono 0.4.45`, `tracing 0.1.44`, `tracing-subscriber 0.3.23`).
    `num-bigint` stays `^0.4` (see Open Questions).

#### 3. `native/ex_scylla/Cargo.lock` (Modify)

*   **Type / Scope**: lock file.
*   **Proposed Changes**: regenerate with `cargo update`.

#### 4. `native/ex_scylla/src/types.rs` and `native/ex_scylla/src/session/types.rs` (Modify)

*   **Type / Scope**: Rust NIF source.
*   **Proposed Changes**: replace `scylla_cql::frame::` with `scylla::frame::` in
    the three imports listed in the Technical Plan. No logic changes.

#### 5. `mix.exs` and `mix.lock` (Modify)

*   **Type / Scope**: Mix project.
*   **Proposed Changes**: `elixir: "~> 1.15"`. Raise `ex_doc` to `~> 0.40.4`,
    `lcov_ex` to `~> 0.3.6` and `erlcass` to `~> 4.1.5`. `rustler`
    (`~> 0.38.0`), `testcontainers` (`~> 2.4`) and `benchee` (`~> 1.5`) are
    already the newest. Run `mix deps.update --all` to refresh `mix.lock`.

#### 6. `test/utils/test_support.ex` (Modify)

*   **Type / Scope**: test helper.
*   **Proposed Changes**: container image `scylladb/scylla:2026.3`.

#### 7. Elixir sources under `lib/` (Modify, only if needed)

*   **Type / Scope**: Elixir library.
*   **Proposed Changes**: fix any new Elixir 1.20 compiler warning reported by
    `mix compile --warnings-as-errors`, without changing behavior.

#### 8. Documentation (Modify)

*   **Type / Scope**: docs and EGM contracts.
*   **Proposed Changes**: update versions and the image tag in `README.md`,
    `AGENTS.md`, `CONTRIBUTING.md`, `docs/architecture.md`,
    `docs/development-guide.md`, `bench/README.md`, `native/README.md` and
    `native/ex_scylla/README.md`. Replace the invariant "`scylla` and
    `scylla-cql` move together" with "do not depend on `scylla-cql` directly;
    import protocol types from `scylla::frame`".

### Verification Plan

*   **Compilation/Build**: `mise install`, `mise run deps:get`,
    `mise run compile`.
*   **Static Analysis & Linting**: `mise run lint` (mix format check, mix
    compile `--warnings-as-errors`, cargo fmt, cargo check, clippy
    `-D warnings`).
*   **Unit/Integration Tests**: `mise run test` (full ExUnit suite against
    `scylladb/scylla:2026.3`). It must pass at the same count as before the
    upgrade.
*   **Benchmarks (optional)**: `mise run bench query` to check for regressions.
    `erlcass` must also build on OTP 29 for the comparison jobs.

### Implementation Findings (2026-11-15)

These came up during implementation and changed or extended the plan above:

1.  **Elixir 1.20 pin-operator warnings.** Six binary matches in
    `lib/ex_scylla/cql_types.ex` used an outer variable in `size(...)` without
    `^`. Elixir 1.20 warns about this, which fails `--warnings-as-errors`.
    Fixed by adding `^`. Behavior is unchanged.
2.  **Rust 1.100 `unused_dependencies` lint.** It flagged `tracing-subscriber`,
    which no source file uses. Removed from `Cargo.toml`.
3.  **Driver behavior change in `use_keyspace`.** With `scylla` 1.10, a failed
    `use_keyspace` (for example, a keyspace that does not exist) removes the
    pool's connections, and the session keeps the bad name for new connections.
    Every query then fails with `No connections in the pool` until a later
    `use_keyspace` succeeds. After that the pool refills in about 100 ms. In
    1.8 the old connections stayed usable. The existing test shared one
    session, so it broke 10 other tests. The test now uses its own session and
    also checks recovery. This is a user-visible change and belongs in the
    release notes.
4.  **`erlcass` does not build on OTP 29** (worked around in `mix.exs`, see item 6). Its Makefile generates
    `c_src/env.mk` with `erl -noshell -s init stop -eval ...`, and OTP 29 no
    longer runs the `-eval` in that order. This breaks `mise run bench`
    (benchmarks only). It needs an upstream `erlcass` fix or a local
    workaround. Not addressed here.
5.  **`hackney` 1.25.0 security advisories.** It is test-only, pulled in by
    `testcontainers ~> 1.20`, and no newer 1.x exists. Not addressed here.
6.  **Follow-up round (same day).**
    *   Pinned `scylla = "=1.10.0"`, because `raw_rows_with_metadata()` is still unstable.
    *   Upgraded `num-bigint` to 0.5.
    *   Fixed the `erlcass` build in `mix.exs`.
    *   Fixed these drift-report bugs: UDT decoding (`field_types`), `:duration` decoding, driver error mapping, error and retry typespecs, two wrong function specs, and macOS link args in test builds.
    *   Investigated pre-serializing on the write path. It is blocked because `SerializedValues` is `pub(crate)`. See `2026-11-15_UPGRADE_REPORT.md`.

--------------------------------------------------------------------------------

*Drafted with EGM*
