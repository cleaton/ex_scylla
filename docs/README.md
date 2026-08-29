# ExScylla Technical Documentation (OpenWiki)

Welcome to the ExScylla OpenWiki technical documentation. This documentation is written for both human developers and AI coding agents to provide clear, concise, and structured insight into the system's design, architecture, I/O patterns, type mapping, and development workflows.

## Table of Contents

- [Architecture & System Design](architecture.md): Overview of the NIF boundary, Tokio async runtime integration, resource lifecycle, and dual-API design.
- [I/O & Data Flow](io-and-data-flow.md): Step-by-step lifecycle of query execution, prepared statements, batches, paging, and streaming.
- [Type System Mapping](type-system.md): Detailed reference mapping between CQL types, Rust `scylla::value::CqlValue`, and Elixir terms.
- [Development & Operations Guide](development-guide.md): Local development workflows with `mise`, running tests with Testcontainers, generating coverage, and running Benchee benchmarks.
- [Release Process Guide](release-process.md): Detailed steps for pre-release validation, version bumps, tagging, and Hex.pm publishing.
- [Agent Guidelines](../AGENTS.md): Fast onboarding handbook and invariant rules for AI coding agents.

## Vision & Design Principles

1. **Minimal Abstraction**: ExScylla provides a direct, unbloated bridge to the official [scylla-rust-driver](https://github.com/scylladb/scylla-rust-driver) without adding redundant translation layers.
2. **Zero-Overhead Asynchrony**: All I/O is scheduled onto a dedicated Tokio multithreaded runtime and returned directly to the calling Elixir PID via Erlang message passing.
3. **Safety & Predictability**: Resources are managed via Rustler's `ResourceArc`, preventing memory leaks and dangling pointers.
4. **Extensibility**: When new features land in the ScyllaDB Rust driver, they can be exposed with minimal boilerplate following the established NIF and macro patterns.
