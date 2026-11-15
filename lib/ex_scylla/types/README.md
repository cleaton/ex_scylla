# ExScylla.Types

## Overview

Elixir type specs and structs that mirror the Rust `scylla` driver types returned
by (or passed to) the NIF layer. Most structs are declared with
`ExScylla.Macros.Native.native_struct/1` (all keys enforced) so the Rust side can
encode them by module name (`#[module = "ExScylla.Types.*"]` attributes under
`native/ex_scylla/src/`). `Row` and `QueryResultRaw` have no Rust binding; they are
built only by the Elixir decoder. `QueryResult` also
holds the Elixir-side row decoder used by the decoded and raw result APIs.

## Interface

- `ExScylla.Types` (`types.ex`): shared type aliases.
  - Opaque resource references: `session`, `session_builder`, `query`, `batch`,
    `prepared_statement`, `execution_profile`, `execution_profile_builder`,
    `execution_profile_handle`, `default_policy_builder`,
    `latency_awareness_builder`, `load_balancing_policy_resource`
    (`load_balancing_policy` aliases it).
  - Enums: `consistency` (`:any | :one | :two | :three | :quorum | :all |
    :local_quorum | :each_quorum | :local_one`), `serial_consistency`
    (`:serial | :local_serial`), `batch_type` (`:logged | :unlogged | :counter`),
    `transport_compression` (`:lz4 | :snappy`), `retry_policy`
    (`:default_retry_policy | :fallthrough_retry_policy`), `pool_size`
    (`{:per_host, n} | {:per_shard, n}`), `speculative_execution_policy`.
  - Result aliases: `metrics`, `cluster_state`, `tracing_info`, `msg`.
  - Values: `value` (tagged tuples such as `{:int, integer()}`, `{:text, String.t()}`,
    `:empty`, `{:list, [value]}`, `{:map, [{value, value}]}`), `values`,
    `column_type`, `ts_micros`, `duration_ms`, `paging_state`, `uuid`,
    `parse_error` (`{:error, {:parse_value, msg}}`).
- `ExScylla.Types.QueryResult` (`native_struct`: `rows`, `rows_count`,
  `column_types`, `warnings`, `tracing_id`, `paging_state`, `serialized_size`):
  - Row decoding matches common fixed-width types and text/ascii/blob directly out of
    the rows binary (private `row_typed/3`, `row_raw/3`); other types go through
    `ExScylla.CqlTypes.decode_value/2`.
  - `decode/1`: when `rows` is a binary, decodes it into `[%Row{columns: [{type, value} | nil]}]`.
  - `decode_raw/1`: decodes into `%QueryResultRaw{rows: [[value | nil]]}` (no type tags).
  - `decode_rows/3,4`, `decode_rows_raw/3,4`, `decode_row/2,3`, `decode_row_raw/2,3`:
    public recursive helpers; each cell is a 32-bit signed length prefix, `-1` = null,
    values delegated to `ExScylla.CqlTypes.decode_value/2`.
- `ExScylla.Types.QueryResultRaw`: Elixir-only plain struct (`@enforce_keys`) with
  the same fields as `QueryResult`, `rows :: [[term()]] | nil`.
- `native_struct` modules: `Row` (`columns`), `Token` (`value`), `CqlDuration`
  (`months`, `days`, `nanoseconds`), `ColumnSpec` (`name`, `table_spec`, `typ`),
  `TableSpec` (`ks_name`, `table_name`), `PartitionKeyIndex` (`index`, `sequence`),
  `PreparedMetadata` (`col_count`, `pk_indexes`, `col_specs`; docs.rs crate
  `:scylla_cql`), `UserDefinedType` (`type_name`, `keyspace`, `fields`),
  `UserDefinedColumnType` (`type_name`, `keyspace`, `field_types`),
  `SimpleSpeculativeExecutionPolicy` (`max_retry_count`, `retry_interval_ms`),
  `PercentileSpeculativeExecutionPolicy` (`max_retry_count`, `percentile`).
- Plain `defstruct` modules (no enforced keys): `ClusterState` (`nodes`,
  `keyspaces`, `cluster_name`), `NodeInfo` (`host_id`, `address`, `datacenter`,
  `rack`), `Metrics` (query/error/retry counters, rates, connection counters,
  `latency_avg_ms`, `latency_99_percentile_ms`), `TracingInfo`, `TracingEvent`.
- Subdirectory: [errors](./errors/README.md) - tagged-tuple error typespecs
  (`QueryError`, `NewSessionError`, `DbError`, and nested errors).

## Invariants

- Struct module names and field names are part of the NIF encoding contract: the
  Rust `#[module = "ExScylla.Types.*"]` bindings under `native/ex_scylla/src/`
  reference these modules by name. Renaming a module or field
  requires the matching Rust change.
- `QueryResult.decode/1` and `decode_raw/1` pass non-binary `rows` through
  unchanged; `rows_count` must equal the number of rows encoded in the binary.
- Decoded runtime shapes differ from the `value` typespec in places (from
  `ExScylla.CqlTypes.decode_value/2`): `:uuid`/`:timeuuid` become lowercase
  hyphenated hex strings, maps become Elixir maps, tuples become Elixir tuples,
  UDTs become maps keyed by field name, and `:duration` becomes
  `%CqlDuration{}` (decoded from three zigzag vints). `{:custom, _}` columns are
  not decoded and come back as raw binaries.
- UDT columns arrive as `{:user_defined_type, %UserDefinedColumnType{}}`; `CqlTypes.decode_value/2`
  also accepts the plain `[{name, type}]` list form.

## Side Effects

- None at runtime (pure data and pure decoding). Compile-time: `use
  ExScylla.Macros.Native` reads `native/ex_scylla/Cargo.lock` to build docs.rs URLs.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
- Decoder unit tests: `mix test test/cql_types_unit_test.exs`
