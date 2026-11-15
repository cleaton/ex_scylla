# Changelog

## [0.11.0] - 2026-11-15

### Breaking changes
- **Minimum versions**: Elixir `~> 1.15` (already required by `rustler` 0.38) and, when building from source, Rust 1.94 (required by `scylla` 1.10).
- **More specific error tags**: driver errors that used to arrive as `{:io_error, msg}` now use their own tags. For example, wrong or missing bound values return `{:bad_query, {:serialize_values_error, ...}}`, and request timeouts return `{:request_timeout, msg}`. New tags: `QueryError` `:request_timeout`; `NewSessionError` `:metadata_error`, `:use_keyspace_error`, `:illegal_config`; `DbError` `:rate_limit_reached`; `BadQuery` `:partition_key_extraction`, `:too_many_queries_in_batch_statement`, `:other`; `PartitionKeyError` `:serialization_error`. `BadKeyspaceName` now returns `:too_long`, and `PartitionKeyError` returns `:no_pk_index_value` where it applies.
- **Malformed bound values no longer raise**: `query`, `execute`, `batch` and the paged variants return `{:error, {:parse_value, msg}}` instead of raising `ArgumentError`.
- **Invalid UUID, `varint` and `decimal` values are rejected**: a `{:uuid, _}`/`{:timeuuid, _}` value that is neither a 16-byte binary nor a UUID string, or a `varint`/`decimal` string that doesn't parse, used to be sent as an empty value without any error. `query`, `execute`, `batch` and the paged variants now return `{:error, {:parse_value, msg}}`, and so does `Prepared.compute_partition_key/2`. The token functions raise `ArgumentError`, as they already do for other malformed values.
- **`use_keyspace` failure leaves the session unusable** (driver 1.10 behavior, scylla-rust-driver #1909): if the server rejects the keyspace (it doesn't exist, wrong case, no permission), every request fails with a connection-pool error until a later `use_keyspace/3` succeeds.
- **`ExScylla.Types.UserDefinedColumnType`**: the field `fields_types` is renamed to `field_types` to match the encoded data.

### Added
- `config :ex_scylla, tokio_worker_threads: n` (read when the native library loads) sets the worker count of the shared Tokio runtime. The default is still one worker per core.
- `{:uuid, _}` and `{:timeuuid, _}` bound values accept UUID strings (`"123e4567-e89b-12d3-a456-426614174000"`, any case) as well as 16-byte binaries.
- `:duration` columns decode to `%ExScylla.Types.CqlDuration{}`.
- Docs (`ExScylla.Session`, README): a process that keeps many `async_*` requests in flight should use `Process.flag(:message_queue_data, :off_heap)`. Replies come from several Tokio threads, and with an on-heap queue they contend for the mailbox lock.

### Changed
- **Toolchain**: Erlang 29.1.2, Elixir 1.20.5-otp-29, Rust 1.100.0, grcov 0.10.8 (`mise.toml`).
- **Rust dependencies**: `scylla` pinned to `=1.10.0` (raw-rows access uses an unstable driver API). The direct `scylla-cql` dependency is removed. `rustler` 0.39.0, `num-bigint` 0.5. `tracing-subscriber` (unused) is removed.
- **Elixir dependencies**: `ex_doc` 0.40.4, `lcov_ex` 0.3.6, `erlcass` 4.1.5 (bench).
- **Bound values for `query`/`execute` (and the paged variants) are decoded off the BEAM scheduler**: they are copied into the NIF's owned environment and decoded on a Tokio worker. The scheduler-side cost of `async_execute` drops from about 6 µs to under 2 µs per request, and on a 4-shard server, 1000 concurrent executes finish about 8% faster. Batches still decode on the scheduler, because copying large value lists cost more than it saved.
- **Faster UUID decoding**: one hex encode per UUID instead of five.
- **Faster row decoding**: common column types (`int`, `bigint`, `timestamp`, `double`, `text`, ...) are matched directly out of the rows binary. Decoding 1000 rows is about 1.55x faster (raw) and 1.6x faster (typed), and allocates less.
- **Tests and benchmarks**: ScyllaDB image `scylladb/scylla:2026.3`. `EX_SCYLLA_TEST_SMP` sets the shard count, and `EX_SCYLLA_TOKIO_WORKERS` (bench env) sets the worker count.

### Fixed
- **The Hex package builds again**: it now ships `native/ex_scylla/.cargo/config.toml` and `build.rs`. Without the config, the `scylla` dependency was compiled without `--cfg scylla_unstable` and the crate failed to build (`no method named raw_rows_with_metadata`) unless the user set `RUSTFLAGS` themselves. 0.10.x packages had the same gap.
- **NULL can be bound**: `nil` in a bound-value list binds NULL (`query`, `execute`, `batch`, the paged variants, `compute_partition_key` and the token functions). Before, `nil` was rejected and the only option was `:empty`, which stores a zero-length value, not NULL.
- **Reading an empty (zero-length) cell no longer crashes**: for types without an empty form (`int`, `uuid`, `timestamp`, ...) it decodes to `nil`, and typed rows show it as a bare `nil`, like NULL. Such cells come from binding `:empty` or from other clients. `text`, `ascii` and `blob` still decode to `""`.
- UDT columns failed to decode, because the column metadata struct and `CqlTypes` disagreed on its shape.
- Typespecs: `:fallthrough_retry_policy` (was `:fall_through_retry_policy`), `DbError` `:unavailable`, `QueryError` `:too_many_orphaned_stream_ids`, and the return specs of `Prepared.compute_partition_key/2` and `ExecutionProfileHandle.map_to_another_profile/2`.
- macOS: test builds keep the `-undefined dynamic_lookup` link args.
- Benchmarks: `erlcass` builds on OTP 29 (`mix.exs` writes its `env.mk`). The insert benchmark's UUIDs are stored again (it used to send strings, which became nulls; both drivers now get the same strings). The query benchmark decodes ExScylla rows (it used to skip decoding, which made ExScylla look faster than it was). Async benchmark jobs use off-heap mailboxes for both drivers, and the insert benchmark truncates its table before each scenario (Benchee runs scenarios alphabetically, so `erlcass` always ran first on an empty table).

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