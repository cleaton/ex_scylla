use std::cell::Cell;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use rustler::ResourceArc;
use scylla::load_balancing::LoadBalancingPolicy;
use scylla::load_balancing::{DefaultPolicy, DefaultPolicyBuilder, LatencyAwarenessBuilder};

pub struct DefaultPolicyBuilderResource(pub Mutex<Cell<DefaultPolicyBuilder>>);
pub struct DefaultPolicyResource(pub DefaultPolicy);
pub struct LatencyAwarenessPolicyBuilderResource(pub Mutex<Cell<LatencyAwarenessBuilder>>);
pub struct LoadBalancingPolicyResource(pub Arc<dyn LoadBalancingPolicy>);

macro_rules! use_builder {
    ($dpbr:ident, $e:expr) => {
        let guard = $dpbr.0.lock().unwrap();
        guard.set($e(guard.take()));
        drop(guard);
    };
}

fn dpb_build(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
) -> ResourceArc<LoadBalancingPolicyResource> {
    let mut guard: MutexGuard<Cell<DefaultPolicyBuilder>> = dpbr.0.lock().unwrap();
    let builder = guard.get_mut().clone();
    drop(guard);
    ResourceArc::new(LoadBalancingPolicyResource(builder.build()))
}
fn dpb_enable_shuffling_replicas(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    enable: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.enable_shuffling_replicas(enable)
    });
    dpbr
}

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

fn dpb_new() -> ResourceArc<DefaultPolicyBuilderResource> {
    ResourceArc::new(DefaultPolicyBuilderResource(Mutex::new(Cell::new(
        DefaultPolicyBuilder::new(),
    ))))
}

fn dpb_permit_dc_failover(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    permit: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.permit_dc_failover(permit)
    });
    dpbr
}

fn dpb_prefer_datacenter(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    datacenter_name: String,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.prefer_datacenter(datacenter_name)
    });
    dpbr
}
fn dpb_prefer_rack(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    rack_name: String,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.prefer_rack(rack_name)
    });
    dpbr
}
fn dpb_token_aware(
    dpbr: ResourceArc<DefaultPolicyBuilderResource>,
    is_token_aware: bool,
) -> ResourceArc<DefaultPolicyBuilderResource> {
    use_builder!(dpbr, |dpb: DefaultPolicyBuilder| {
        dpb.token_aware(is_token_aware)
    });
    dpbr
}
