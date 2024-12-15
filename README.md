# ExScylla

* Minimal interface & abstraction
* * Easy to maintain & to add new features as they arrive in the rust driver
* * Users of ExScylla should be able to leverage the rust driver documentation

SessionBuilder, Session + statements Batch, Query & Prepared uses Arc references to the underlying Rust object.
Set functions that mutates such objects will return a new reference to an updated copy.
As old instances are not immidetly removed (wait for Elixir GC), these functions should be avoided in the hot path. Better create the object once and use it multiple times.

Docs and api example found at: https://hexdocs.pm/ex_scylla

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
```

## Installation
https://hex.pm/packages/ex_scylla

## Contributing

We welcome contributions! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## Changelog

For a detailed list of changes and updates, please refer to the [CHANGELOG.md](CHANGELOG.md) file.

```elixir
def deps do
  [
    {:ex_scylla, "~> 0.1.0"}
  ]
end
```

