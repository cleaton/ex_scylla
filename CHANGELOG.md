# Changelog

## [0.10.2] - 2026-08-29

### Added
- **Scylla Rust Driver 1.8.0 & `scylla-cql` 1.8.0**: Upgraded core driver dependency with latest upstream features, bug fixes, and stability improvements.
- **Mise Toolchain Management**: Added [`mise.toml`](mise.toml) managing Erlang (28.4.1), Elixir (1.19.5-otp-28), Rust (1.97.1), `grcov` (0.10.7), and development tasks (`compile`, `test`, `test:coverage`, `bench`, `scylla:run`, `scylla:start`, `scylla:stop`).
- **Cluster Metadata Enhancement**: Exposed `cluster_name` field in `ExScylla.Types.ClusterState` populated from native `Session.get_cluster_state/1`.
- **Datacenter & Rack Locality APIs**: Added `prefer_datacenter/2`, `prefer_datacenter_and_rack/3`, and `prefer_no_datacenter/1` in `ExScylla.SessionBuilder`.
- **Comprehensive Test Coverage**: Achieved >99% line and >85% function coverage for Elixir, and >84% line and >88% function coverage for Rust NIFs. Added `rust_full_coverage_test.exs`, `cql_types_unit_test.exs`, and `session_full_coverage_test.exs`.
- **Agent Guidelines & OpenWiki Docs**: Added [`AGENTS.md`](AGENTS.md) and [`docs/`](docs/) detailing architecture, I/O data flows, type system, and development guidelines.
- **Automated Rust Code Coverage**: Added LLVM profile instrumentation and `grcov` integration to `mise run test:coverage`.

### Changed
- **Rustler 0.38.0 Upgrade**: Migrated all NIF resource structs to implement `rustler::Resource` and registered them explicitly with `env.register::<T>()`.
- **ScyllaDB 2026.2 Test Environment**: Updated Testcontainers to `scylladb/scylla:2026.2` with `NetworkTopologyStrategy` keyspace replication for full tablet compatibility.
- **Removed Legacy Files**: Removed deprecated `Makefile`, `.tool-versions`, and `run_scylla.sh`.

## [0.5.1]
### Added
- Integration of TestContainers for managing ScyllaDB instances during testing and benchmarking.

### Changed
- Bumped version to 0.5.1 to reflect the addition of TestContainers. 