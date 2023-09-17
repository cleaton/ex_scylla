use std::cell::Cell;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;

use rustler::ResourceArc;
use scylla::load_balancing::DefaultPolicy;
use scylla::load_balancing::LoadBalancingPolicy;
use scylla::load_balancing::{DefaultPolicyBuilder, LatencyAwarenessBuilder};

pub struct DefaultPolicyBuilderResource(pub Mutex<Cell<DefaultPolicyBuilder>>);
pub struct LatencyAwarenessPolicyBuilderResource(pub Mutex<Cell<LatencyAwarenessBuilder>>);
pub struct LoadBalancingPolicyResource(pub Arc<dyn LoadBalancingPolicy>);

macro_rules! use_builder {
    ($dpbr:ident, $e:expr) => {
        let guard = $dpbr.0.lock().unwrap();
        guard.set($e(guard.take()));
        drop(guard);
    };
}

macro_rules! use_builder_lab {
    ($labr:ident, $e:expr) => {
        let guard = $labr.0.lock().unwrap();
        guard.set($e(guard.take()));
        drop(guard);
    };
}

#[rustler::nif]
fn dpb_build(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
) -> ResourceArc<LoadBalancingPolicyResource> {
    let mut guard: MutexGuard<Cell<DefaultPolicyBuilder>> = dpbr.0.lock().unwrap();
    let builder = guard.get_mut().clone();
    drop(guard);
    ResourceArc::new(LoadBalancingPolicyResource(builder.build()))
}

#[rustler::nif]
fn dpb_enable_shuffling_replicas(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    enable: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.enable_shuffling_replicas(enable)
    });
    dpbr
}

#[rustler::nif]
fn dpb_latency_awareness(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    latency_awareness_builder: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    let mut guard: MutexGuard<Cell<LatencyAwarenessBuilder>> =
        latency_awareness_builder.0.lock().unwrap();
    let builder = guard.get_mut().clone();
    drop(guard);
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.latency_awareness(builder)
    });
    dpbr
}

#[rustler::nif]
fn dpb_new() -> ResourceArc<DefaultPolicyBuilderResource> {
    ResourceArc::new(DefaultPolicyBuilderResource(Mutex::new(Cell::new(
        DefaultPolicyBuilder::new(),
    ))))
}

#[rustler::nif]
fn dpb_permit_dc_failover(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    permit: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.permit_dc_failover(permit)
    });
    dpbr
}

#[rustler::nif]
fn dpb_prefer_datacenter(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    datacenter_name: String,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.prefer_datacenter(datacenter_name)
    });
    dpbr
}

#[rustler::nif]
fn dpb_prefer_rack(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    rack_name: String,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.prefer_rack(rack_name)
    });
    dpbr
}

#[rustler::nif]
fn dpb_token_aware(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    is_token_aware: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.token_aware(is_token_aware)
    });
    dpbr
}


#[rustler::nif]
fn lab_exclusion_threshold(
    labr: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
    exclusion_threshold: f64,
) -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    use_builder_lab!(labr, |lab: LatencyAwarenessBuilder| {
        lab.exclusion_threshold(exclusion_threshold)
    });
    labr
}

#[rustler::nif]
fn lab_minimum_measurements(
    labr: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
    minimum_measurements: usize,
) -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    use_builder_lab!(labr, |lab: LatencyAwarenessBuilder| {
        lab.minimum_measurements(minimum_measurements)
    });
    labr
}

#[rustler::nif]
fn lab_new() -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    ResourceArc::new(LatencyAwarenessPolicyBuilderResource(Mutex::new(Cell::new(LatencyAwarenessBuilder::new()))))
}

#[rustler::nif]
fn lab_retry_period(
    labr: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
    retry_period_ms: u64,
) -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    use_builder_lab!(labr, |lab: LatencyAwarenessBuilder| {
        lab.retry_period(Duration::from_millis(retry_period_ms))
    });
    labr
}

#[rustler::nif]
fn lab_scale(
    labr: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
    scale_ms: u64,
) -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    use_builder_lab!(labr, |lab: LatencyAwarenessBuilder| {
        lab.scale(Duration::from_millis(scale_ms))
    });
    labr
}

#[rustler::nif]
fn lab_update_rate(
    labr: ResourceArc<LatencyAwarenessPolicyBuilderResource>,
    update_rate_ms: u64,
) -> ResourceArc<LatencyAwarenessPolicyBuilderResource> {
    use_builder_lab!(labr, |lab: LatencyAwarenessBuilder| {
        lab.update_rate(Duration::from_millis(update_rate_ms))
    });
    labr
}

#[rustler::nif]
fn dp_default() -> ResourceArc<LoadBalancingPolicyResource> {
    ResourceArc::new(LoadBalancingPolicyResource(Arc::new(DefaultPolicy::default())))
}

