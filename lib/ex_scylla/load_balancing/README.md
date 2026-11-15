# ExScylla.LoadBalancing

## Overview

Wrappers for building the driver's default load-balancing policy. Builders are
opaque NIF resources configured through chained `native_f` calls; the built
policy is passed to `ExScylla.Execution.ExecutionProfileBuilder.load_balancing_policy/2`.

## Interface

- `ExScylla.LoadBalancing.DefaultPolicy` (prefix `:dp`): `default/0` returns a
  `T.load_balancing_policy()` from `ExScylla.Native.dp_default/0`.
- `ExScylla.LoadBalancing.DefaultPolicyBuilder` (prefix `:dpb`): `new/0`,
  `build/1` (returns `T.load_balancing_policy()`), `enable_shuffling_replicas/2`,
  `latency_awareness/2` (takes a `T.latency_awareness_builder()`),
  `permit_dc_failover/2`, `prefer_datacenter/2`, `prefer_datacenter_and_rack/3`,
  `token_aware/2`.
- `ExScylla.LoadBalancing.LatencyAwarenessBuilder` (prefix `:lab`): `new/0`,
  `exclusion_threshold/2` (`number()`), `minimum_measurements/2`,
  `retry_period/2` (ms), `scale/2` (ms), `update_rate/2` (ms).

## Invariants

- Builder setters mutate the Rust builder in place under a `Mutex` and return the
  same reference (`use_builder!` for `DefaultPolicyBuilder`, `use_builder_lab!`
  for `LatencyAwarenessBuilder`, both in
  `native/ex_scylla/src/execution/load_balancing.rs`). Earlier references observe
  the change, so a builder must not be shared and configured differently from two
  places.
- `DefaultPolicyBuilder.latency_awareness/2` stores a clone of the latency builder,
  so later `LatencyAwarenessBuilder` calls do not affect it. `build/1` also clones
  the builder, which stays usable after building.
- Each wrapper must have a matching `ExScylla.Native` stub and Rust NIF in
  `native/ex_scylla/src/execution/load_balancing.rs`.

## Side Effects

- Allocates NIF resources only. No network or disk I/O.

## Verification

- Run: `mise run test` (requires Docker; ScyllaDB via Testcontainers)
- Targeted: `mix test test/execution/load_balancing_test.exs`
