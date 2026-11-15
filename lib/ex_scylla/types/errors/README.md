# ExScylla.Types.Errors

## Overview

Typespec-only modules that describe the tagged-tuple error terms produced by the
Rust NIF layer (`native/ex_scylla/src/errors.rs`, `NifTaggedEnum` derives). Each
module maps to one Rust driver error enum and links to its docs.rs page through
`ExScylla.Macros.Native.scylla_version/0`. No module here defines functions or
structs; they exist only for `@type t` contracts used in return specs elsewhere.

## Interface

| Module | Maps to | `@type t` tags |
| --- | --- | --- |
| `ExScylla.Types.Errors.QueryError` | `scylla::errors::ExecutionError` | `:db_error` (`DbError.t()`), `:bad_query` (`BadQuery.t()`), `:io_error`, `:protocol_error`, `:invalid_message`, `:timeout_error`, `:too_many_oprhaned_stream_ids`, `:unable_to_alloc_stream_id`, `:translation_error` (`TranslationError.t()`) |
| `ExScylla.Types.Errors.NewSessionError` | `scylla::errors::NewSessionError` | `:failed_to_resolve_any_hostname`, `:empty_known_nodes_list`, `:db_error`, `:bad_query`, `:io_error`, `:protocol_error`, `:invalid_message`, `:timeout_error`, `:too_many_orphaned_stream_ids`, `:unable_to_alloc_stream_id`, `:translation_error` |
| `ExScylla.Types.Errors.DbError` | `scylla::errors::DbError` | `:syntax_error`, `:invalid`, `:already_exists`, `:function_failure`, `:authentication_error`, `:unauthorized`, `:config_error`, `:unavaliable`, `:overloaded`, `:is_bootstrapping`, `:truncate_error`, `:read_timeout`, `:write_timeout`, `:read_failure`, `:write_failure`, `:unprepared`, `:server_error`, `:protocol_error`, `:other` |
| `ExScylla.Types.Errors.BadQuery` | `scylla::errors::BadQuery` | `:serialize_values_error` (`SerializeValuesError.t()`), `:values_too_long_for_key`, `:bad_keyspace_name` (`BadKeyspaceName.t()`) |
| `ExScylla.Types.Errors.BadKeyspaceName` | `scylla::errors::BadKeyspaceName` | `:empty`, `:too_long`, `:illegal_character` |
| `ExScylla.Types.Errors.SerializeValuesError` | `scylla::serialize::SerializationError` | `:too_many_values`, `:mixing_named_and_not_named_values`, `:value_too_big`, `:parse_error` |
| `ExScylla.Types.Errors.PartitionKeyError` | `scylla::statement::prepared::PartitionKeyError` | `:no_pk_index_value`, `:value_too_long` |
| `ExScylla.Types.Errors.TranslationError` | `scylla::errors::TranslationError` | `:invalid_address_in_rule`, `:no_rule_for_address` |

Every tag carries either a `msg :: String.t()` or the nested error type shown.

Rust can also emit tags that the specs leave out: `QueryError` and `NewSessionError`
`:request_timeout`, `DbError` `:rate_limit_reached`, and `BadQuery` `:other`.

The Rust conversions in `errors.rs` only produce a subset of the listed tags:

- `QueryError`: `:db_error`, `:bad_query`, or `:io_error` (every other driver error).
- `NewSessionError`: `:failed_to_resolve_any_hostname`, `:empty_known_nodes_list`,
  or `:io_error`.
- `BadQuery`: never `:bad_keyspace_name` (unmapped variants become `:other`).
- `SerializeValuesError`: only `:parse_error`.
- `BadKeyspaceName`: never `:too_long`.
- `PartitionKeyError`: only `:value_too_long`.

## Invariants

- Tags must match the atoms emitted by the Rust `NifTaggedEnum` enums in
  `native/ex_scylla/src/errors.rs` (snake_case of the variant name). When a Rust
  variant is added or renamed, update the matching `@type t` here.
- Known spec drift (as of this snapshot): `DbError` spells `:unavaliable` while Rust
  emits `:unavailable` (`ScyllaDbError::Unavailable`), and `QueryError` spells
  `:too_many_oprhaned_stream_ids` while Rust emits `:too_many_orphaned_stream_ids`.
- `@typedoc` URLs are built at compile time from `Native.scylla_version()`, which
  reads `native/ex_scylla/Cargo.lock`.

## Side Effects

- None at runtime. Compile-time only: indirect `Cargo.lock` read via
  `ExScylla.Macros.Native`.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
