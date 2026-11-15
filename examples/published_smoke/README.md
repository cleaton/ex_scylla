# published_smoke

A standalone Mix project that installs `ex_scylla` from Hex and runs
[`smoke.exs`](./smoke.exs) against a real ScyllaDB. It checks the package users
get, not the local source: the Hex package must compile its NIF on its own and
the basic session API must work.

## Run

```bash
mise run verify:published            # version from the repo's mix.exs
mise run verify:published 0.10.2     # any published version
```

The task copies this directory to a temporary directory outside the repository
(so the repo's `.cargo/config.toml` and build artifacts can't leak in), unsets
`RUSTFLAGS`, and runs `mix deps.get` and `mix run smoke.exs`. If nothing listens
on `127.0.0.1:9042`, it starts ScyllaDB with `mise run scylla:start` and stops it
afterwards. Set `SCYLLA_NODE=host:port` to use another node.

`smoke.exs` checks:
- `ex_scylla` came from Hex at the expected version, and the NIF loads;
- connecting, `CREATE KEYSPACE`/`TABLE`, a prepared insert with a UUID string
  and a NULL, and reading the row back decoded and raw;
- a malformed value returns `{:error, {:parse_value, _}}`.
