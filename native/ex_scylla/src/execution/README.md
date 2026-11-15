# execution (Execution profiles and driver policies)

## Overview
NIF handlers and resources for configuring per-request execution behavior: execution profiles (builder, profile, and handle), load-balancing policies (`DefaultPolicy` and latency awareness), retry policies, speculative execution policies, and connection pool size. `mod.rs` declares all submodules. `crate::session_builder` and the statement modules consume the resulting resources.

## Interface
- `mod.rs`: `pub mod` declarations for the files below.
- `execution_profile.rs`
  - `ExecutionProfileResource(pub ExecutionProfile)`.
  - NIFs: `ep_builder/0`, `ep_into_handle/1`, `ep_into_handle_with_label/2`, `ep_to_builder/1`.
- `execution_profile_builder.rs`
  - `ExecutionProfileBuilderResource(pub Mutex<ExecutionProfileBuilder>)`, which implements `Deref`.
  - NIFs: `epb_build/1`, `epb_consistency/2`, `epb_load_balancing_policy/2`, `epb_request_timeout/2` (ms, optional), `epb_retry_policy/2`, `epb_serial_consistency/2`, `epb_speculative_execution_policy/2`.
- `execution_profile_handle.rs`
  - `ExecutionProfileHandleResource(pub ExecutionProfileHandle)`.
  - NIFs: `eph_map_to_another_profile/2`, `eph_pointee_to_builder/1`.
- `load_balancing.rs`
  - Resources: `DefaultPolicyBuilderResource(Mutex<Cell<DefaultPolicyBuilder>>)`, `LatencyAwarenessPolicyBuilderResource(Mutex<Cell<LatencyAwarenessBuilder>>)`, `LoadBalancingPolicyResource(Arc<dyn LoadBalancingPolicy>)`.
  - Default policy builder NIFs: `dpb_new/0`, `dpb_build/1`, `dpb_enable_shuffling_replicas/2`, `dpb_latency_awareness/2`, `dpb_permit_dc_failover/2`, `dpb_prefer_datacenter/2`, `dpb_prefer_datacenter_and_rack/3`, `dpb_token_aware/2`.
  - Latency awareness builder NIFs: `lab_new/0`, `lab_exclusion_threshold/2`, `lab_minimum_measurements/2`, `lab_retry_period/2`, `lab_scale/2`, `lab_update_rate/2` (durations in ms).
  - `dp_default/0`.
- `retry_policy.rs`: `ScyllaRetryPolicy` (`NifUnitEnum`: `DefaultRetryPolicy | FallthroughRetryPolicy`) -> `Arc<dyn RetryPolicy>`.
- `speculative_execution.rs`
  - `ScyllaSimpleSpeculativeExecutionPolicy` (`ExScylla.Types.SimpleSpeculativeExecutionPolicy`): `max_retry_count`, `retry_interval_ms`.
  - `ScyllaPercentileSpeculativeExecutionPolicy` (`ExScylla.Types.PercentileSpeculativeExecutionPolicy`): `max_retry_count`, `percentile`.
  - `ScyllaSpeculativeExecutionPolicy` (untagged union of the two) -> `Arc<dyn SpeculativeExecutionPolicy>`.
- `pool_size.rs`: `ScyllaPoolSize` (`NifTaggedEnum`: `{:per_host, n} | {:per_shard, n}`) -> `scylla::client::PoolSize`.

## Invariants
- **Builders are mutated in place and return the same `ResourceArc`.** `epb_*` uses a `use_builder!` that does `std::mem::take` under the `Mutex`. `dpb_*` and `lab_*` use `use_builder!` and `use_builder_lab!` with `Cell::take`/`set`. This is unlike the clone-and-return-new pattern for statements. A shared builder reference observes every mutation.
- `epb_build` and `dpb_build` clone the builder (so it stays usable) and enter the shared Tokio runtime (`crate::runtime::rt().enter()`) before calling `build()`, because the driver may spawn tasks during construction. The runtime must already be initialized by `runtime::init()` in `../lib.rs` `load/2`.
- `dpb_latency_awareness` clones the latency-awareness builder, so later `lab_*` calls do not affect an already-configured default policy builder.
- `eph_map_to_another_profile` mutates the shared handle target. Every statement or session holding a clone of that `ExecutionProfileHandle` sees the new profile.
- `ScyllaPoolSize` panics (`expect("invalid per-host pool size")` / `"invalid per-shard pool size"`) if the value cannot convert to the driver's non-zero size type, for example `0`.
- All six resource types in this directory must stay registered in `../lib.rs` `load/2`.

## Side Effects
- `epb_build` and `dpb_build` may spawn background tasks on the shared Tokio runtime (for example latency-awareness updaters inside the driver).
- `eph_map_to_another_profile` changes the behavior of in-flight and future requests that use the handle.
- No direct network or disk I/O.

## Verification
- Run: `mise run lint` and `mise run test` (requires Docker).
