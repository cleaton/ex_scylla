# src (ex_scylla NIF crate sources)

## Overview
Rust sources for the `ex_scylla` cdylib, loaded by Elixir as `Elixir.ExScylla.Native` (`rustler::init!` in `lib.rs`). The crate is a thin layer over the `scylla` driver crate. It exposes driver objects as opaque Rustler resources, converts values between Elixir terms and CQL types, and runs all network I/O on one shared multi-threaded Tokio runtime. Results go back to the calling Elixir process as messages.

Each top-level `<name>.rs` holds the NIF handlers for one area. The matching `<name>/` subdirectory holds its resource and conversion types.

## Interface
- `lib.rs`: `rustler::init!("Elixir.ExScylla.Native", load = load)`. `load/2` calls `runtime::init(worker_threads)` (the load data from `ExScylla.NativeConfig.load_data/0`: an integer or `nil`) and registers 12 resource types: `SessionBuilderResource`, `SessionResource`, `ScyllaRawRowsResource`, `BatchResource`, `PreparedStatementResource`, `QueryResource`, `ExecutionProfileBuilderResource`, `ExecutionProfileHandleResource`, `ExecutionProfileResource`, `DefaultPolicyBuilderResource`, `LatencyAwarenessPolicyBuilderResource`, and `LoadBalancingPolicyResource`. Public modules: `consts`, `errors`, `runtime`, `types`, `utils`.
- `runtime.rs`: `RUNTIME: OnceLock<Runtime>`, `init(worker_threads: Option<usize>)` (multi-thread runtime with time and IO enabled; Tokio's default of one worker per core unless `:tokio_worker_threads` is configured), and `rt() -> &'static Runtime` (panics with `"runtime is not initialized"` if called before `init`).
- `utils.rs`
  - Traits: `ToElixir<T>` (with `IS_UNWRAPPED`) and `ToRust<T>`.
  - `ScyllaResult<R, E>`, which encodes `Unwrapped(r)` as `r`, `Ok(r)` as `{:ok, r}`, and `Err(e)` as `{:error, e}`.
  - Macros: `async_elixir!`, `async_elixir_decoded!`, `to_elixir!`, `clone_enum!`.
  - Blanket impls: `Result<S, E>` -> `ScyllaResult`, `()` -> `:ok`, `bool`, and `Vec<B>`.
- `types.rs`: unit enums mirrored from the driver via `clone_enum!`.
  - `ScyllaConsistency`: `Any, One, Two, Three, Quorum, All, LocalQuorum, EachQuorum, Serial, LocalSerial, LocalOne`.
  - `ScyllaTransportCompression`: `Lz4, Snappy`.
  - `ScyllaSerialConsistency`: `Serial, LocalSerial`.
- `consts.rs`: atoms `not_implemented_yet` and `execute`.
- `errors.rs`: driver error -> Elixir term translation via `to_elixir!`.
  - `ScyllaQueryError` (from `ExecutionError`): `attempt_error/1` maps a request attempt's error to `DbError`, `BadQuery` (value serialization), `UnableToAllocStreamId`, `InvalidMessage` (frame parse errors), `ProtocolError`, or `IoError` (broken connection). `RequestTimeout` maps directly, and a failed prepare uses its first attempt's error. Connection pool, empty plan and metadata errors stay `IoError`.
  - `ScyllaDbError` (one variant per `DbError` kind, each carrying the server message; unknown kinds -> `Other`).
  - `ScyllaNewSessionError` maps every driver variant (`FailedToResolveAnyHostname`, `EmptyKnownNodesList`, `MetadataError`, `UseKeyspaceError`, `IllegalConfig`). `ScyllaBadKeyspaceName` maps `Empty`, `TooLong` and `IllegalCharacter`. `ScyllaPartitionKeyError` maps `PartitionKeyExtraction` to `NoPkIndexValue`, `TokenCalculation` to `ValueTooLong` and `Serialization` to `SerializationError`. `ScyllaQueryError::TimeoutError`/`TooManyOrphanedStreamIds`/`TranslationError` are kept for compatibility but not produced.
  - Also: `ScyllaBadQuery`, `ScyllaTranslationError`, `ScyllaSerializeValuesError`, `ScyllaNewSessionError`, `ScyllaBadKeyspaceName`, `ScyllaPartitionKeyError`, and `ScyllaError(Atom, String)` (`parse/1` -> `{:parse_value, msg}`, `Into<rustler::Error>`).
- `session.rs`: `Session` NIFs.
  - Async (reply via message): `s_query`, `s_query_paged`, `s_execute`, `s_execute_paged`, `s_batch`, `s_prepare`, `s_prepare_batch`, `s_use_keyspace`, `s_refresh_metadata`, `s_await_schema_agreement`, `s_await_timed_schema_agreement`, `s_check_schema_agreement`, `s_fetch_schema_version`, `s_get_tracing_info`.
  - Sync: `s_calculate_token`, `s_calculate_token_for_partition_key`, `s_get_cluster_state`, `s_get_default_execution_profile_handle`, `s_get_keyspace`, `s_get_metrics`.
- `session_builder.rs`: `sb_new`, async `sb_build`, and in-place setters: `sb_known_node(s)(_addr)`, `sb_compression`, `sb_connection_timeout`, `sb_pool_size`, `sb_user`, `sb_use_keyspace`, `sb_default_execution_profile_handle`, keepalive/TCP options, schema-agreement options, tracing-info fetch options, `sb_prefer_datacenter(_and_rack)`, `sb_prefer_no_datacenter`, `sb_write_coalescing`, `sb_disallow_shard_aware_port`, `sb_fetch_schema_metadata`, `sb_keyspaces_to_fetch`.
- `query.rs`: `q_new`, `q_get_*` / `q_set_*` for consistency, serial consistency, idempotence, page size, request timeout, timestamp, tracing, and execution profile handle; `q_with_page_size`; `q_get/set_retry_policy` (return `:not_implemented_yet`).
- `prepared_statement.rs`: `ps_compute_partition_key`, `ps_get_id`, `ps_get_statement`, `ps_get_keyspace_name`, `ps_get_table_name`, `ps_get_prepared_metadata`, `ps_get_prepare_tracing_ids`, `ps_is_confirmed_lwt`, `ps_is_token_aware`, `ps_get/set_use_cached_result_metadata`, plus the same `get/set` families as `query.rs` (there is no `with_page_size`, and `ps_get_page_size` returns `Option<i32>`); `ps_get/set_retry_policy` (return `:not_implemented_yet`).
- `batch.rs`: `b_new`, `b_new_with_statements`, `b_append_statement`, `b_get_type`, plus `get/set` for consistency, serial consistency, idempotence, request timeout, timestamp, tracing, and execution profile handle; `b_get/set_retry_policy` (return `:not_implemented_yet`).
- `test_builder.rs`: not declared as a module in `lib.rs`, so it is not compiled.
- Subdirectories:
  - [batch/](./batch/README.md): `BatchResource`, `ScyllaBatchType`.
  - [execution/](./execution/README.md): execution profiles, load balancing, retry, speculative execution, and pool size.
  - [prepared_statement/](./prepared_statement/README.md): `PreparedStatementResource`, `ScyllaPreparedMetadata`, `ScyllaPartitionKeyIndex`.
  - [query/](./query/README.md): `QueryResource`.
  - [session/](./session/README.md): `SessionResource`, `ScyllaValue`, `ScyllaQueryResult`, and the cluster, tracing, and metrics structs.
  - [session_builder/](./session_builder/README.md): `SessionBuilderResource`.

## Invariants
- **No blocking in schedulers.** Network operations use `async_elixir!(env, opaque, future[, |env, res| encoder])`. The macro does the following:
  1. Captures `env.pid()`.
  2. Saves `opaque` in an `OwnedEnv`.
  3. Spawns the future on `runtime::rt()`.
  4. Replies with `{opaque, result}` via `send_and_clear`.

  The NIF returns `:ok` right away. Never use `block_on`.
- `s_query`, `s_execute` and their `_paged` variants use `async_elixir_decoded!` (`utils.rs`): the bound values are copied into the `OwnedEnv` and decoded on the Tokio worker, not the BEAM scheduler. A malformed value is replied as `{opaque, {:error, {:parse_value, msg}}}` instead of raising. `s_batch` decodes synchronously (copying large value lists into the owned env was slower in benchmarks) and returns `{:error, {:parse_value, msg}}` directly from the NIF call. Other async NIFs also decode synchronously (for example `s_get_tracing_info` raises `invalid_uuid` on a bad id).
- Statement resources (`Query`, `PreparedStatement`, `Batch`) are copy-on-write: setters `to_owned()` the inner value and return a new `ResourceArc`. The exception is `{q,ps,b}_set_retry_policy` (and the matching getters), which return the atom `:not_implemented_yet`. `b_set_retry_policy` takes an extra unused `bool` argument. Builder resources (`SessionBuilder`, the execution-profile builder, and the load-balancing builders) are mutated in place under a `Mutex` and return the same `ResourceArc`.
- `sb_build` takes the builder out of its `Cell`, leaving a default `SessionBuilder` in the resource.
- `s_query`, `s_execute`, and `s_batch` use the driver's unpaged APIs (`query_unpaged`, `execute_unpaged`, `batch`). `s_query_paged` and `s_execute_paged` fetch a single page and return `paging_state` only when more pages exist. A `nil` input paging state means `PagingState::start()`.
- `s_fetch_schema_version` gets the version by calling `session.await_schema_agreement()`, so it waits for agreement.
- `s_calculate_token` and `s_calculate_token_for_partition_key` return `nil` on driver error instead of an error tuple, but raise when value decoding fails. `s_calculate_token` ignores its session argument.
- Success payloads differ by NIF. `s_await_schema_agreement`, `s_refresh_metadata` and `s_use_keyspace` reply with a bare `:ok`. `s_await_timed_schema_agreement` replies `{:ok, true}` and `s_check_schema_agreement` replies `{:ok, boolean}`.
- Error payloads differ by NIF. `s_query*`, `s_execute*`, and `s_batch` return a structured `ScyllaQueryError`. `sb_build` returns a structured `ScyllaNewSessionError`, and `ps_compute_partition_key` raises a `ScyllaPartitionKeyError` term synchronously. `s_prepare`, `s_prepare_batch`, `s_use_keyspace`, `s_refresh_metadata`, the schema-agreement NIFs, and `s_get_tracing_info` return `{:error, String}`. `s_await_timed_schema_agreement` returns `{:error, "timeout"}` on timeout.
- Every NIF here must have a matching declaration in `lib/ex_scylla/native.ex` and a public wrapper in `lib/ex_scylla/` (dual-layer sync, see `AGENTS.md`).
- The crate must be compiled with `--cfg scylla_unstable` (set by `../build.rs` and `../.cargo/config.toml`).

## Side Effects
- `lib.rs` `load/2` starts a process-global multi-threaded Tokio runtime when the NIF library loads. The runtime lives for the life of the VM.
- `sb_build` opens TCP connections to ScyllaDB/Cassandra nodes. The session then runs background metadata, keepalive, and pool tasks on the shared runtime.
- Async `s_*` NIFs send CQL requests over the network and send exactly one message `{opaque, result}` to the calling PID when they finish.
- `s_use_keyspace` and `s_refresh_metadata` change session-wide state. A failed `s_use_keyspace` leaves the session with no usable connections until a later switch succeeds. The driver stores the name before validating it, and since 1.10 it removes connections that fail the switch.
- Result rows are handed to Elixir as resource-backed binaries (`ScyllaRawRowsResource`). The bytes stay alive as long as Elixir references the binary.

## Verification
- Run: `mise run lint` (cargo fmt check, `cargo check`, `cargo clippy -D warnings` with `--cfg scylla_unstable`) and `mise run test` (ExUnit integration suite; requires Docker for Testcontainers ScyllaDB).
