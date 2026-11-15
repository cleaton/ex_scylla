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
| `ExScylla.Types.Errors.QueryError` | `scylla::errors::ExecutionError` | `:db_error` (`DbError.t()`), `:bad_query` (`BadQuery.t()`), `:io_error`, `:protocol_error`, `:invalid_message`, `:timeout_error`, `:too_many_orphaned_stream_ids`, `:unable_to_alloc_stream_id`, `:request_timeout`, `:translation_error` (`TranslationError.t()`) |
| `ExScylla.Types.Errors.NewSessionError` | `scylla::errors::NewSessionError` | `:failed_to_resolve_any_hostname`, `:empty_known_nodes_list`, `:metadata_error`, `:use_keyspace_error`, `:illegal_config`, `:io_error`, plus the `QueryError` tags (`:db_error`, `:bad_query`, `:protocol_error`, `:invalid_message`, `:timeout_error`, `:too_many_orphaned_stream_ids`, `:unable_to_alloc_stream_id`, `:request_timeout`, `:translation_error`) |
| `ExScylla.Types.Errors.DbError` | `scylla::errors::DbError` | `:syntax_error`, `:invalid`, `:already_exists`, `:function_failure`, `:authentication_error`, `:unauthorized`, `:config_error`, `:unavailable`, `:overloaded`, `:is_bootstrapping`, `:truncate_error`, `:read_timeout`, `:write_timeout`, `:read_failure`, `:write_failure`, `:unprepared`, `:server_error`, `:protocol_error`, `:rate_limit_reached`, `:other` |
| `ExScylla.Types.Errors.BadQuery` | `scylla::errors::BadQuery` | `:serialize_values_error` (`SerializeValuesError.t()`), `:values_too_long_for_key`, `:bad_keyspace_name` (`BadKeyspaceName.t()`), `:partition_key_extraction`, `:too_many_queries_in_batch_statement`, `:other` |
| `ExScylla.Types.Errors.BadKeyspaceName` | `scylla::errors::BadKeyspaceName` | `:empty`, `:too_long`, `:illegal_character` |
| `ExScylla.Types.Errors.SerializeValuesError` | `scylla::serialize::SerializationError` | `:too_many_values`, `:mixing_named_and_not_named_values`, `:value_too_big`, `:parse_error` |
| `ExScylla.Types.Errors.PartitionKeyError` | `scylla::statement::prepared::PartitionKeyError` | `:no_pk_index_value`, `:value_too_long`, `:serialization_error` |
| `ExScylla.Types.Errors.TranslationError` | `scylla::errors::TranslationError` | `:invalid_address_in_rule`, `:no_rule_for_address` |

Every tag carries either a `msg :: String.t()` or the nested error type shown.

How `errors.rs` maps driver errors (`attempt_error/1` handles single request
attempts):

- `QueryError`:
  - server errors become `:db_error`
  - value serialization errors become `{:bad_query, {:serialize_values_error, {:parse_error, msg}}}`
  - frame parse errors become `:invalid_message`
  - other protocol-level failures become `:protocol_error`
  - `:unable_to_alloc_stream_id` and `:request_timeout` map one to one
  - failed prepares use the error of their first attempt
  - connection-level failures (broken connection, connection pool, empty plan, metadata) stay `:io_error`
- `NewSessionError`: every driver variant has its own tag. Anything else becomes `:io_error`.
- `SerializeValuesError`: only `:parse_error` is produced. The other tags are kept for compatibility.
- `:timeout_error`, `:too_many_orphaned_stream_ids` and `:translation_error` are not produced by driver 1.10. They remain in the specs for compatibility.

## Invariants

- Tags must match the atoms emitted by the Rust `NifTaggedEnum` enums in
  `native/ex_scylla/src/errors.rs` (snake_case of the variant name). When a Rust
  variant is added or renamed, update the matching `@type t` here.
- `@typedoc` URLs are built at compile time from `Native.scylla_version()`, which
  reads `native/ex_scylla/Cargo.lock`.

## Side Effects

- None at runtime. Compile-time only: indirect `Cargo.lock` read via
  `ExScylla.Macros.Native`.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
- Targeted: `mix test test/session_full_coverage_test.exs` ("driver errors map to specific tags")
