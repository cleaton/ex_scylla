# I/O & Data Flow

This document details the lifecycle of read, write, batch, paging, and streaming requests flowing through ExScylla.

## Request Execution Flow

```
[Elixir Caller Process]
       |
       | 1. Session.execute(session, prepared, values)
       v
[Macro Layer: ExScylla.Macros.Native]
       |
       | 2. Creates opaque reference: ref = make_ref()
       | 3. Calls Native.s_execute(ref, session, prepared, values)
       v
[Rustler NIF: s_execute]
       |
       | 4. async_elixir! captures PID and ref in OwnedEnv
       | 5. Spawns future onto Tokio runtime
       v
[Tokio Runtime & Scylla Driver]
       |
       | 6. Sends serialized CQL request to cluster node
       | 7. Awaits response frame from ScyllaDB
       v
[Result Encoding]
       |
       | 8. Converts CqlValue rows to ScyllaQueryResult / ScyllaRow
       | 9. OwnedEnv::send_and_clear sends {ref, {:ok, result}} to Elixir PID
       v
[Elixir Caller Process]
       |
       | 10. Receives {^ref, result}, post-processes via QueryResult.decode/1
       v
[Caller receives {:ok, %QueryResult{rows: [...]}}]
```

## Statement Execution Types

### 1. Unprepared Query (`Session.query/3`)
- Direct execution of CQL text.
- Suitable for DDL statements (e.g. `CREATE KEYSPACE`, `CREATE TABLE`) and low-frequency queries.

### 2. Prepared Statements (`Session.prepare/2` + `Session.execute/3`)
- Pre-parsed and token-aware statements on the ScyllaDB cluster.
- Computes partition keys on the client to route queries directly to replica nodes.
- Recommended for all high-throughput read and write paths.

### 3. Batches (`Session.batch/3` & `Session.prepare_batch/2`)
- Combines multiple `INSERT`, `UPDATE`, or `DELETE` statements.
- Supported batch types: `:logged`, `:unlogged`, and `:counter`.
- Prepared batches validate statement tokens for optimal server-side execution.

## Paging and Streaming

### Paged Queries (`Session.query_paged/4`, `Session.execute_paged/4`)
- Manually requests a single page of results.
- Returns `%QueryResult{rows: [...], paging_state: pgs}` where `pgs` is an opaque binary token.
- Subsequent calls pass `pgs` to fetch the next chunk.

### Streaming (`Session.query_stream/3`, `Session.execute_stream/3`)
- Wraps paged execution inside an Elixir `Stream.resource/3`.
- Transparently fetches subsequent pages as rows are consumed by the stream pipeline.
- Example:
```elixir
Session.query_stream(session, "SELECT * FROM my_keyspace.my_table", [])
|> Stream.take(500)
|> Enum.each(&process_row/1)
```
