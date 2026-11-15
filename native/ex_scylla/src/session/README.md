# session (Session resource, CQL value conversion, and result types)

## Overview
Type definitions for the `crate::session` NIF module. The NIF handlers (`s_*`) live in the parent file [`../session.rs`](../session.rs). This directory holds `types.rs`, the largest conversion module in the crate. It defines:
- the `Session` and raw-rows resources,
- the bidirectional `CqlValue` <-> Elixir tagged-tuple mapping (`ScyllaValue`),
- result, metadata, cluster, tracing, and metrics structs returned to Elixir,
- the polymorphic statement inputs (`ScyllaQuery`, `ScyllaBatchStatement`).

Other modules import from here: `prepared_statement` (`ScyllaColumnSpec`), `../session_builder.rs` (`ScyllaSocketAddr`), `../batch.rs` (`ScyllaBatchStatement`), and `../prepared_statement.rs`.

## Interface
- **Resources**
  - `SessionResource(pub Session)`: wraps `scylla::client::session::Session`. `impl ToElixir<ResourceArc<SessionResource>> for Session`.
  - `ScyllaRawRowsResource(pub bytes::Bytes)`: owns raw row bytes so they can be exposed to Elixir as a resource binary without copying.
- **Values**
  - `ScyllaValue` (`NifTaggedEnum`): `Ascii, Boolean, Blob, Counter, Decimal, Date, Double, Duration, Empty, Float, Int, BigInt, Text, Timestamp, Inet, List, Map, Set, UserDefinedType, SmallInt, TinyInt, Time, Timeuuid, Tuple, Uuid, Varint`. Elixir form is `{:tag, value}`, for example `{:int, 42}`.
    - `impl TryFrom<CqlValue> for ScyllaValue`: returns `Err(rustler::Error::Term("unsupported_cql_value"))` for unmapped variants.
    - `impl TryFrom<ScyllaValue> for CqlValue` (see Invariants).
  - `ScyllaBinary(pub Vec<u8>)`: custom `Encoder`/`Decoder` for Elixir binaries. Implements `From<Vec<u8>>`, `From<bytes::Bytes>`, and `From<uuid::Uuid>`.
  - `ScyllaUuid`: an `Encoder` that turns a UUID into a 16-byte binary.
  - `ScyllaPageState`: `Encoder`/`Decoder` between `scylla::response::PagingState` and an Elixir binary, plus `ToRust` for `Option<ScyllaPageState>`.
  - `ScyllaCqlDuration` (`ExScylla.Types.CqlDuration`): `months`, `days`, `nanoseconds`.
  - `ScyllaIpAddr` (untagged `IPv4(u8 x4)` / `IPv6(u16 x8)` tuples) and `ScyllaSocketAddr` (`{ip_tuple, port}`, `From<ScyllaSocketAddr> for SocketAddr`).
  - `ScyllaUserDefinedType` (`ExScylla.Types.UserDefinedType`): `type_name`, `keyspace`, `fields`.
- **Results and metadata**
  - `ScyllaQueryResult<'a>` (`ExScylla.Types.QueryResult`): `rows` (raw row bytes as a resource binary, or `nil`), `rows_count`, `column_types`, `warnings`, `tracing_id`, `paging_state`, `serialized_size`. Built by `ScyllaQueryResult::new(env, QueryResult)`.
  - `ScyllaColumnType` (`NifTaggedEnum`, snake_case renames such as `:big_int`, `:user_defined_type`), plus `From<ColumnType<'_>>`. Unmapped types become `Custom(String)` (`{:custom, "<Debug repr>"}` or `{:custom, "Unknown"}`).
  - `ScyllaColumnSpec` (`ExScylla.Types.ColumnSpec`), `ScyllaTableSpec` (`ExScylla.Types.TableSpec`), and `ScyllaUserDefinedColumnType` (`ExScylla.Types.UserDefinedColumnType`). `impl ToElixir<ScyllaColumnSpec> for ColumnSpec<'_>`.
  - `ScyllaToken` (`ExScylla.Types.Token`): `value: i64`.
  - Helper impls: `ToElixir<String> for String` and `ToElixir<ScyllaUuid> for uuid::Uuid`.
- **Cluster, observability, and metrics**
  - `ScyllaClusterState` (`ExScylla.Types.ClusterState`): `nodes`, `keyspaces`, `cluster_name`.
  - `ScyllaNodeInfo` (`ExScylla.Types.NodeInfo`): `host_id`, `address`, `datacenter`, `rack`.
  - `ScyllaTracingInfo` (`ExScylla.Types.TracingInfo`) and `ScyllaTracingEvent` (`ExScylla.Types.TracingEvent`).
  - `ScyllaMetrics` (`ExScylla.Types.Metrics`): query/error/retry counters, rates, connection and timeout counts, and latency.
- **Statement inputs**
  - `ScyllaQuery` (`NifTaggedEnum`): `String(String) | QueryResource(ResourceArc<QueryResource>)`, with `From<ScyllaQuery> for Query`.
  - `ScyllaBatchStatement` (`NifUntaggedEnum`): `String | QueryResource | PreparedStatement`, with `From` for `BatchStatement`.

## Invariants
- **The Rust layer does not decode rows.** `ScyllaQueryResult::new` copies only the column types and exposes the raw rows frame (`raw_rows_with_metadata().raw_rows()`) as a resource binary via `ResourceArc::make_binary`. The Elixir side (`QueryResult.decode/1` and `decode_raw/1` in `lib/ex_scylla/types/query_result.ex`) decodes rows. For non-rows results, `rows` and `rows_count` are `None`.
- `ScyllaQueryResult::new` always sets `paging_state: None` and `serialized_size: 0`. Paged NIFs in `../session.rs` fill in `paging_state` afterwards when the driver returns `PagingStateResponse::HasMorePages`.
- `ScyllaValue` <-> `CqlValue` must stay bidirectional, and must stay in sync with `lib/ex_scylla/cql_types.ex` (see `AGENTS.md`). Decimal and Varint travel as decimal strings (`BigDecimal` / `BigInt`).
- Elixir -> CQL conversion is `impl TryFrom<ScyllaValue> for CqlValue` (`Error = String`). `Uuid`/`Timeuuid` accept a 16-byte binary or a UUID string (`parse_uuid`, `uuid::Uuid::try_parse_ascii`). An unparsable `Decimal`/`Varint` string or UUID, including one nested in a collection, tuple or UDT, is an error; `../session.rs` returns it as `{:error, {:parse_value, msg}}`.
- Bound-value lists decode each element as `Option<ScyllaValue>` (`../session.rs` `BoundValues = Vec<Option<CqlValue>>`): `nil` binds NULL, while `ScyllaValue::Empty` (`:empty`) binds `CqlValue::Empty`, a zero-length value that is not NULL.
- Every `NifStruct` `#[module = ...]` and its field names must match the Elixir struct in `lib/ex_scylla/types/`.
- `ScyllaRawRowsResource` and `SessionResource` must stay registered in `../lib.rs` `load/2`.

## Side Effects
- None in `types.rs` itself, apart from Erlang-heap allocation for binaries. The network I/O for `query`, `execute`, `batch`, `prepare`, schema agreement, tracing, and `use_keyspace` happens in `../session.rs`, on the shared Tokio runtime via `async_elixir!`.

## Verification
- Run: `mise run lint` and `mise run test` (requires Docker).
