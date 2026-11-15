# config

## Overview

Compile-time Mix configuration for ExScylla. `config.exs` is the entry point
and delegates to one per-environment file selected by `config_env()`. The
per-environment files currently set no application keys; the library reads no
configuration from here at runtime.

## Interface

- `config.exs`: `import Config` followed by `import_config "#{config_env()}.exs"`.
- `dev.exs`: used when `MIX_ENV=dev`. Contains no `Config` calls (its only
  content is the bare expression `1`).
- `test.exs`: used when `MIX_ENV=test`. Empty file.
- `bench.exs`: used when `MIX_ENV=bench` (set by the `bench` alias in
  `mix.exs`, `run_bench/1`). Contains only `import Config`.

## Invariants

- Every Mix environment the project is compiled under must have a matching
  `<env>.exs` file here, because `config.exs` imports
  `"#{config_env()}.exs"` without a fallback. There is no `prod.exs`, so
  compiling ExScylla itself with `MIX_ENV=prod` fails at config load. (When
  ExScylla is a dependency, this directory is not loaded.)
- `:ex_scylla, :test_node` is not set here. `test/test_helper.exs` sets it at
  runtime with `Application.put_env/3`, and `lib/` doctests read it with
  `Application.get_env(:ex_scylla, :test_node, "127.0.0.1:9042")`.

## Side Effects

- None. These files are evaluated by Mix at config load and set no
  application environment keys.

## Verification

- Run: `mise run test`
