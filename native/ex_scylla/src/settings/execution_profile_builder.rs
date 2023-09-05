use crate::types::*;
use rustler::ResourceArc;
use scylla::execution_profile::ExecutionProfileBuilder;
use scylla::speculative_execution::SpeculativeExecutionPolicy;
use scylla::statement::SerialConsistency;
use std::cell::Cell;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use super::execution_profile::ExecutionProfileResource;
use super::load_balancing::LoadBalancingPolicyResource;
use super::retry_policy::ScyllaRetryPolicy;

pub struct ExecutionProfileBuilderResource(pub Mutex<Cell<ExecutionProfileBuilder>>);

impl Deref for ExecutionProfileBuilderResource {
    type Target = Mutex<Cell<ExecutionProfileBuilder>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! use_builder {
    ($epbr:ident, $e:expr) => {
        let guard = $epbr.lock().unwrap();
        guard.set($e(guard.take()));
        drop(guard);
    };
}

fn epb_build(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
) -> ResourceArc<ExecutionProfileResource> {
    let mut guard: MutexGuard<Cell<ExecutionProfileBuilder>> = epbr.0.lock().unwrap();
    let epbc = guard.get_mut().clone();
    drop(guard);
    ResourceArc::new(ExecutionProfileResource(epbc.build()))
}

fn epb_consistency(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.consistency(consistency.into())
    });
    epbr
}

fn epb_load_balancing_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    load_balancing_policy: ResourceArc<LoadBalancingPolicyResource>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.load_balancing_policy(load_balancing_policy.0.clone())
    });
    epbr
}

fn epb_request_timeout(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    timeout_ms: Option<u64>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.request_timeout(timeout_ms.map(|ms| Duration::from_millis(ms)))
    });
    epbr
}

fn epb_retry_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    retry_policy: ScyllaRetryPolicy,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.retry_policy(retry_policy.into())
    });
    epbr
}

fn epb_serial_consistency(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    serial_consistency: Option<ScyllaSerialConsistency>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.serial_consistency(serial_consistency.map(|ssc| {
            let sc: SerialConsistency = ssc.into();
            sc
        }))
    });
    epbr
}

fn epb_speculative_execution_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    speculative_execution_policy: Option<Arc<dyn SpeculativeExecutionPolicy>>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    use_builder!(epbr, |epb: ExecutionProfileBuilder| {
        epb.speculative_execution_policy(speculative_execution_policy.into())
    });
    epbr
}
