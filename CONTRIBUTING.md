# Contributing to ExScylla

## Prerequisites
- [mise](https://mise.jdx.dev/) (manages Erlang, Elixir, and Rust toolchains)
- Docker (required for TestContainers and local database workflows)

## Development Environment

1. Fork and clone the repository
2. Install runtimes with `mise`:
   ```bash
   mise install
   ```
3. Install dependencies:
   ```bash
   mise run deps:get
   ```
4. Compile the project and native extensions:
   ```bash
   mise run compile
   ```
5. Install test dependencies for code coverage:
   ```bash
   mise run install-test-deps
   ```

## Testing

Tests use TestContainers to automatically manage ScyllaDB instances (`scylladb/scylla:2026.2`). Ensure Docker is running before executing tests.

```bash
# Run the full test suite
mise run test

# Run a specific test file
mix test test/session_test.exs

# Run a specific test (line number)
mix test test/session_test.exs:42

# Run tests with coverage (LLVM profile + Elixir coverage)
mise run test:coverage
```

Test coverage reports are generated automatically and saved to the `cover/` directory.

For comprehensive architectural patterns and invariants, see [docs/](docs/README.md) and [AGENTS.md](AGENTS.md).

## Benchmarks

Benchmarks help measure performance impacts of changes. They also use TestContainers to manage ScyllaDB instances.

```bash
# Run query benchmark
mix bench query

# Run streaming benchmark
mix bench query_stream
```

Benchmark files are located in the `bench/` directory. Each benchmark:
- Automatically starts a ScyllaDB container
- Sets up necessary schema
- Runs performance measurements
- Provides comparison metrics

## Adding a New Feature

1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. If adding new Rust functionality:
   - Add the feature to `native/ex_scylla/src/`
   - Expose it through the NIF interface
   - Add corresponding Elixir wrapper in `lib/ex_scylla/`

3. If adding new Elixir functionality:
   - Add implementation in appropriate module under `lib/ex_scylla/`
   - Follow existing patterns for error handling and type specifications

4. Add tests:
   - Unit tests in appropriate `test/` directory
   - Integration tests if needed
   - Consider adding benchmark if performance-critical

5. Update documentation:
   - Add/update module and function documentation
   - Update README.md if adding major features
   - Add examples in documentation

6. Ensure all tests pass:
   ```bash
   mix test
   ```

7. Run benchmarks if changes might affect performance:
   ```bash
   mix bench query
   ```

8. Commit your changes:
   ```bash
   git commit -m "Add: descriptive message about your changes"
   ```

9. Push to your fork and create a Pull Request

## Code Style

- Follow existing code style and patterns
- Use typespecs for public functions
- Include documentation with examples
- Keep commits focused and atomic
- Write descriptive commit messages

## Questions?

Feel free to open an issue for:
- Questions about implementation
- Discussion about potential features
- Reporting bugs
- Seeking clarification about contribution guidelines
