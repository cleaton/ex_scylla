# Development & Operations Guide

This guide covers local development workflows, task automation via `mise`, testing with Testcontainers, code coverage generation, and performance benchmarking.

## Prerequisites

- [mise-en-place](https://mise.jdx.dev/): Runtime and task manager.
- [Docker](https://www.docker.com/): Required for spinning up ScyllaDB containers in tests and benchmarks.

## Environment Setup

Install all language runtimes (Erlang, Elixir, Rust) specified in `mise.toml`:
```bash
mise install
```

Fetch Hex dependencies:
```bash
mise run deps:get
```

Compile native Rust extensions and Elixir modules:
```bash
mise run compile
```

Run linting and compiler checks (mix format, warnings-as-errors, cargo check, clippy):
```bash
mise run lint
```

Auto-format all code (Elixir + Rust):
```bash
mise run format
```

Or run the composite build task (fetches deps, runs lint checks, and compiles):
```bash
mise run build
```

## Running Tests

Tests use Testcontainers to automatically orchestrate isolated ScyllaDB instances (`scylladb/scylla:2026.2`). Ensure Docker is running.

```bash
# Run full test suite
mise run test

# Run a specific test file
mix test test/session_test.exs

# Run a specific test line
mix test test/session_test.exs:42
```

## Code Coverage

Generate Elixir and native Rust LLVM branch coverage reports:
```bash
mise run test:coverage
```
Coverage artifacts and summaries will be written to `cover/`.

## Benchmarking

Benchmarks measure throughput, latency, and resource consumption comparing ExScylla to other drivers.

```bash
# Query execution benchmark
mise run bench query

# Insert throughput benchmark
mise run bench insert

# Large result set retrieval
mise run bench large_result

# Streaming cursor benchmark
mise run bench query_stream
```

## Standalone ScyllaDB for Manual Testing

To run a persistent ScyllaDB 2026.2 container for local experimentation or iex sessions:

```bash
# Foreground container
mise run scylla:run

# Background container
mise run scylla:start

# Stop container
mise run scylla:stop
```
