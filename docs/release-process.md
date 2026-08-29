# Release Process Guide

This guide details the step-by-step procedure for releasing a new version of ExScylla.

---

## 1. Pre-Release Checklist

Before triggering a release, ensure all checks pass cleanly on a clean working tree:

1. **Verify dependencies and clean build**:
   ```bash
   mise run build
   ```

2. **Run full test suite with coverage**:
   ```bash
   mise run test:coverage
   ```
   - Ensure all doctests and unit tests pass with **0 failures**.
   - Verify Elixir and Rust line and function coverage exceed the 80% threshold.

3. **Run performance benchmarks**:
   ```bash
   mise run bench query
   mise run bench insert
   ```
   - Verify that there are no regressions compared to baseline benchmarks.

---

## 2. Version Bump Locations

Ensure the version string matches across all project metadata files:

1. **[`mix.exs`](../mix.exs)**:
   ```elixir
   def project do
     [
       app: :ex_scylla,
       version: "0.10.2",
       ...
     ]
   end
   ```

2. **[`native/ex_scylla/Cargo.toml`](../native/ex_scylla/Cargo.toml)**:
   ```toml
   [package]
   name = "ex_scylla"
   version = "0.10.2"
   ```

3. **[`README.md`](../README.md)**:
   ```elixir
   def deps do
     [
       {:ex_scylla, "~> 0.10.2"}
     ]
   end
   ```

4. **[`CHANGELOG.md`](../CHANGELOG.md)**:
   - Add a new section for `## [X.Y.Z] - YYYY-MM-DD` detailing Added, Changed, Deprecated, Removed, and Fixed items.

---

## 3. Git Commit and Tagging

1. **Commit version changes**:
   ```bash
   git add mix.exs native/ex_scylla/Cargo.toml native/ex_scylla/Cargo.lock README.md CHANGELOG.md
   git commit -m "chore(release): release v0.10.2"
   ```

2. **Create a signed Git tag**:
   ```bash
   git tag -a v0.10.2 -m "Release v0.10.2"
   ```

3. **Push commits and tags to remote**:
   ```bash
   git push origin main --tags
   ```

---

## 4. Publishing to Hex.pm

1. **Validate package packaging and docs**:
   ```bash
   mix hex.build
   ```

2. **Publish the package to Hex**:
   ```bash
   mix hex.publish
   ```
   - If publishing docs separately:
     ```bash
     mix hex.publish docs
     ```

---

## 5. Post-Release Verification

1. Verify package availability on [Hex.pm/packages/ex_scylla](https://hex.pm/packages/ex_scylla).
2. Verify documentation generation on [HexDocs.pm/ex_scylla](https://hexdocs.pm/ex_scylla).
3. Test installing the published dependency in a sample Elixir project:
   ```elixir
   {:ex_scylla, "~> 0.10.2"}
   ```
