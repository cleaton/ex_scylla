# bench

## Overview

Benchee performance scripts that measure ExScylla against a live, throwaway
ScyllaDB container. Three of the four scripts also run the same workload
through the `erlcass` (DataStax C++ driver) NIF so the two can be compared.
These scripts are not part of the library and are not compiled into it. Each
one is a standalone `.exs` file that `mix run` executes in the `:bench` Mix
environment.

## Interface

Each script is a top-level `.exs` file. None of them defines a module. Run one
with `mise run bench <name>`, where `<name>` is the filename without `.exs`.
That task calls the `mix bench` alias (`mix.exs:run_bench/1`), which runs
`mix run bench/<name>.exs` with `MIX_ENV=bench`.

| Script | Keyspace / table | Benchee jobs | Inputs / settings |
| --- | --- | --- | --- |
| `insert.exs` | `load_test_erlcass_insert.test_table` (14 typed columns, PK `col1`) | `erlcass_async`, `exscylla_async` (`Session.async_execute` per row), `erlcass_batch`, `exscylla_batch` (`Batch.new_with_statements(:unlogged, ...)` + `Session.async_batch`) | `Small (1)`, `Medium (100)`, `Large (1000)`; `time: 10`, `parallel: 1` |
| `query.exs` | `load_test_erlcass.test_table` (same 14-column schema, seeded with 100 rows via `:erlcass.execute`) | `erlcass`, `exscylla` (`Session.async_execute`), `exscylla_raw` (`Session.async_execute_raw`) | `Small (1)`, `Medium (100)`, `Large (1000)`; `time: 10`, `parallel: 5` |
| `large_result.exs` | `bench_large.test_table_simple` (PK `(pk, ck)`, seeded with 1000 rows under `pk = 1`) | `erlcass`, `exscylla (typed)` (`Session.execute`), `exscylla_raw (raw)` (`Session.execute_raw`) | Each iteration selects 1000 rows; `time: 10`, `warmup: 2`, `memory_time: 2`, `parallel: 1` |
| `query_stream.exs` | `load_test.simple_table` (`id text`, `value int`, 100 rows) | `async_query` (`Session.async_execute` of a prepared point select). ExScylla only, no erlcass job | `1 query`, `100 queries`, `1000 queries`; `time: 10`, `memory_time: 2`, `parallel: 4` |

The scripts depend on these external symbols:

- `ExScylla.TestSupport.start_container/0` and `setup_simple_keyspace/2`
  ([test/utils/test_support.ex](../test/utils/test_support.ex)). They are
  compiled in only because `mix.exs:elixirc_paths/1` adds `test/utils` for
  `MIX_ENV` `:test` and `:bench`.
- `ExScylla.Session`: `query/3`, `prepare/2`, `execute/3`, `execute_raw/3`,
  `async_execute/3`, `async_execute_raw/3`, `async_batch/3`, `use_keyspace/3`.
- `ExScylla.Statement.Batch.new_with_statements/2`.
- `:erlcass`: `add_prepare_statement/2`, `async_execute/2`, `execute/2`,
  `bind_prepared_statement/1`, `bind_prepared_params_by_index/2`,
  `batch_async_execute/3`. Also `:erlcass_uuid.gen_random/0` and
  `gen_time/0`, which `insert.exs` and `query.exs` use to make UUIDs for both
  drivers.

## Invariants

- Scripts only run under `MIX_ENV=bench`. `benchee` and `erlcass` are
  `only: [:bench]` dependencies, `testcontainers` is `only: [:test, :bench]`
  (`mix.exs:deps/0`), and `TestSupport` is compiled only in those
  environments.
- `mix bench` takes exactly one argument. The name must match an existing
  `bench/<name>.exs`. If it does not, the alias prints an error and runs
  nothing (`mix.exs:run_bench/1`).
- ExScylla async replies arrive as `{opaque, result}`. The default opaque is
  `{function_name, make_ref()}` (`lib/ex_scylla/macros/native.ex`).
  `query.exs` and `query_stream.exs` therefore match on `{{:execute, _}, ...}`
  and `{{:execute_raw, _}, ...}`. `insert.exs` matches on the returned tag
  with `{^tag, _}`. If the default opaque shape changes, these receive
  patterns must change too.
- ExScylla values are bound as tagged tuples such as `{:text, v}`,
  `{:int, v}`, and `{:big_int, v}`. erlcass values are bound as plain terms.
  In `insert.exs` the two row generators (`generate_exscylla_row`,
  `generate_erlcass_row`) must keep the same column order as the
  `INSERT ... VALUES (?, ...)` statement.
- `erlcass` 4.1.5 does not build on OTP 29 by itself: its Makefile generates
  `c_src/env.mk` with `erl -s init stop -eval ...`, and OTP 29 stops the VM
  before the `-eval` runs. `mix.exs:erlcass_build_env/0` writes that file to
  `_build/bench/erlcass_env.mk` and passes `C_SRC_ENV` to the `mix run`
  subprocess. A harmless "Error! Failed to eval" line still appears in the build
  output. Running `MIX_ENV=bench mix compile` directly bypasses the workaround.
- `query_stream.exs` does not use a paging or streaming API, despite its name.
  It measures concurrent `async_execute` point selects.

- Environment knobs (read by `ExScylla.TestSupport` and `config/bench.exs`):
  `EX_SCYLLA_TEST_SMP` sets the ScyllaDB shard count (default `1`), and
  `EX_SCYLLA_TOKIO_WORKERS` sets ExScylla's Tokio worker thread count.
  A single shard funnels both drivers through one connection, which hides the
  Rust driver's shard-aware routing. Compare with `EX_SCYLLA_TEST_SMP=4` too.
- Async jobs call `Process.flag(:message_queue_data, :off_heap)` for both drivers.
  Replies come from several threads, and on-heap queues add lock contention.
  `insert.exs` truncates its table in `before_scenario`, because Benchee runs
  scenarios alphabetically and the first one would otherwise get an empty table.
- ExScylla `async_*` replies carry undecoded rows. `query.exs` decodes them with
  `QueryResult.decode/1` / `decode_raw/1` so both drivers do the same work, and
  `insert.exs` passes the UUID strings `erlcass` generates to both drivers unchanged
  (ExScylla accepts UUID strings since 0.11.0).

## Side Effects

- Each run uses Testcontainers to start a new `scylladb/scylla:2026.3` Docker
  container (`TestSupport.start_container/0`). Docker must be running.
- The scripts create keyspaces and tables with `CREATE ... IF NOT EXISTS` and
  insert seed rows. Those are network writes to the container.
- `insert.exs`, `query.exs`, and `large_result.exs` set the `:erlcass`
  application env (`:keyspace`, `:cluster_options`) and start the `:erlcass`
  application.
- `query.exs` and `query_stream.exs` create a named ETS table, `:exscylla`.
- The scripts print the node address, Benchee reports, and any unexpected
  messages (`IO.inspect`) to stdout.

## Verification

- Run a benchmark: `mise run bench <name>` (`insert`, `query`,
  `large_result`, or `query_stream`)
- Project test suite: `mise run test`
