# ExScylla

An Elixir wrapper around the [scylla-rust-driver](https://rust-driver.docs.scylladb.com/) using Rustler.

* Minimal interface & abstraction
* Easy to maintain & to add new features as they arrive in the rust driver
* Users of ExScylla should be able to leverage the rust driver documentation

Docs and api example found at: https://hexdocs.pm/ex_scylla

## Prerequisites

* **Rust and Cargo**: Since this library uses Rustler to compile the native Rust extension, you must have the Rust toolchain installed.
* **scylla_unstable flag**: Some features of the underlying Rust driver may require the `scylla_unstable` cfg flag to be enabled during compilation.

## Architecture & Performance

### NIF/Rustler & Tokio Runtime
ExScylla uses Rustler to interface with the `scylla-rust-driver`. Because the Rust driver is heavily asynchronous and relies on the Tokio runtime, ExScylla spins up a dedicated Tokio runtime in the background to handle the driver's futures.

### Resource Reference Model & Cloning
`SessionBuilder`, `Session`, and statements (`Batch`, `Query`, `Prepared`) use `Arc` references to the underlying Rust objects. These references are passed to Elixir as NIF resources. 

**Performance Note**: Setter functions that mutate these objects will return a new reference to an updated copy (cloning the underlying Rust struct). As old instances are not immediately removed (waiting for Elixir GC), these mutating functions should be avoided in the hot path. It is better to create and configure the object once, then reuse it multiple times.

### Async API & Timeouts
The library uses asynchronous NIFs for non-blocking operations to avoid starving the Erlang schedulers. By default, async NIF calls have a 5-second default timeout.

### Dual API (Decoded vs Raw)
ExScylla provides a dual API approach:
* **Decoded API**: Automatically translates ScyllaDB types into Elixir types (e.g., maps, lists, tuples).
* **Raw API**: Allows bypassing the translation layer on the Rust side to pass raw result frames directly to Elixir, which can be useful for performance optimization or custom decoding.

## Example

```elixir
alias ExScylla.SessionBuilder
alias ExScylla.Session
alias ExScylla.Types.QueryResult

{:ok, session} = SessionBuilder.new()
                 |> SessionBuilder.known_node("127.0.0.1:9042")
                 |> SessionBuilder.build()
{:ok, ps} = Session.prepare(session, "INSERT INTO test.s_doc (a, b, c) VALUES (?, ?, ?)")
values = [{:text, "test"}, {:int, 2}, {:double, 1.0}]
{:ok, %QueryResult{}} = Session.execute(session, ps, values)

# Iterative queries (streams)
Session.query_stream(session, "SELECT * FROM test.s_doc", [])
|> Enum.each(fn row -> IO.inspect(row) end)
```

## Installation
https://hex.pm/packages/ex_scylla

```elixir
def deps do
  [
    {:ex_scylla, "~> 0.9.1"}
  ]
end
```

## Contributing

We welcome contributions! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## Changelog

For a detailed list of changes and updates, please refer to the [CHANGELOG.md](CHANGELOG.md) file.
