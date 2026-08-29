# Architecture & System Design

ExScylla is designed as a minimal, high-performance bridge between Elixir / BEAM applications and ScyllaDB, leveraging the official asynchronous `scylla-rust-driver` via Rustler NIFs.

## High-Level Architecture

```
+-------------------------------------------------------------+
|                        Elixir / BEAM                         |
|  ExScylla.Session / Batch / Query / Prepared / ExecutionProfile|
+------------------------------+------------------------------+
                               | (NIF calls / Erlang Terms)
                               v
+-------------------------------------------------------------+
|                      Rustler NIF Layer                      |
|           ResourceArc<T>, async_elixir!, Type Translation   |
+------------------------------+------------------------------+
                               | (Tokio Spawns / Rust Futures)
                               v
+-------------------------------------------------------------+
|                    Dedicated Tokio Runtime                  |
|                   scylla Rust Driver (v1.8.0)               |
+------------------------------+------------------------------+
                               | (CQL Binary Protocol v4)
                               v
+-------------------------------------------------------------+
|                     ScyllaDB Cluster Nodes                  |
+-------------------------------------------------------------+
```

## Core Architectural Components

### 1. Tokio Runtime Isolation (`native/ex_scylla/src/runtime.rs`)
The ScyllaDB Rust driver is inherently asynchronous and Tokio-based. ExScylla initializes a static multithreaded Tokio runtime on startup (`runtime::init()`).
- All asynchronous I/O and CQL driver interactions run on Tokio worker threads.
- Erlang schedulers are never blocked waiting on driver futures.

### 2. Resource Reference Model (`ResourceArc<T>`)
Complex Rust types are held in memory as `ResourceArc<T>` pointers:
- `Session` (`SessionResource`)
- `SessionBuilder` (`SessionBuilderResource`)
- `PreparedStatement` (`PreparedStatementResource`)
- `Query` (`QueryResource`)
- `Batch` (`BatchResource`)
- `ExecutionProfile`, `ExecutionProfileBuilder`, `ExecutionProfileHandle`
- `DefaultPolicyBuilder`, `LatencyAwarenessPolicyBuilder`, `LoadBalancingPolicy`

Elixir processes receive opaque references (`#Reference<...>`). The BEAM garbage collector tracks references and deallocates native resources safely when no references remain.

### 3. Immutability Pattern
In accordance with Elixir's functional semantics:
- Mutating operations (e.g. `Query.set_consistency/2`, `Prepared.set_page_size/2`) clone the underlying Rust structure and return a new reference.
- **Guideline**: Configure queries and prepared statements during setup rather than re-creating/mutating them in hot execution loops.

### 4. Asynchronous NIF Bridge (`async_elixir!`)
All network interactions use the `async_elixir!` macro:
1. The Elixir caller generates an opaque reference (token).
2. The NIF receives the calling process PID and the token.
3. The future is spawned onto the Tokio runtime with an `OwnedEnv`.
4. Upon completion, results are encoded into Erlang terms and sent as a message `{token, result}` to the calling PID.

### 5. Dual Result APIs (Decoded vs Raw)
- **Decoded API** (`query`, `execute`, `batch`): Translates `CqlValue` into rich Elixir types (lists, maps, tuples, decimals, UUIDs).
- **Raw API** (`query_raw`, `execute_raw`, `batch_raw`): Returns raw frame binaries wrapped in `ExScylla.Types.QueryResultRaw`, skipping term construction for high-throughput or custom deserialization scenarios.
