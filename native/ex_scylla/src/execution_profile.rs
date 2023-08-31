use std::sync::Arc;
use std::time::Duration;

use rustler::{NifStruct, NifUnitEnum, NifUntaggedEnum};
use rustler::ResourceArc;
use scylla::execution_profile::ExecutionProfileBuilder;
use scylla::ExecutionProfile;
use scylla::load_balancing::DefaultPolicy;
use scylla::load_balancing::LoadBalancingPolicy;
use scylla::retry_policy::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};
use scylla::speculative_execution::{
    PercentileSpeculativeExecutionPolicy, SimpleSpeculativeExecutionPolicy,
};
use scylla::statement::SerialConsistency;
use scylla::transport::downgrading_consistency_retry_policy::DowngradingConsistencyRetryPolicy;
use scylla::transport::speculative_execution::SpeculativeExecutionPolicy;

use types::*;

use crate::types::{ScyllaConsistency, ScyllaSerialConsistency};
use crate::utils::{ToElixir, ToRust};

pub mod types;

#[rustler::nif]
fn ep_builder() -> ResourceArc<ExecutionProfileBuilderResource> {
    ResourceArc::new(ExecutionProfileBuilderResource(ExecutionProfile::builder()))
}

#[rustler::nif]
fn ep_request_timeout(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    timeout_ms: u64,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb: ExecutionProfileBuilder = epbr.0.to_owned();
    epb = epb.request_timeout(Option::from(Duration::from_millis(timeout_ms)));
    epb.ex()
}

#[rustler::nif]
fn ep_consistency(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb = epbr.0.to_owned();
    // print the consistency
    println!("consistency: {:?}", consistency);
    epb = epb.consistency(consistency.into());
    epb.ex()
}

#[rustler::nif]
fn ep_serial_consistency(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    serial_consistency: ScyllaSerialConsistency,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb = epbr.0.to_owned();
    epb = epb.serial_consistency(Option::from(<ScyllaSerialConsistency as Into<SerialConsistency>>::into(serial_consistency)));
    epb.ex()
}

#[rustler::nif]
fn ep_load_balancing_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    policy: ScyllaLoadBalancingPolicy,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb = epbr.0.to_owned();
    let load_balancing_policy = policy.r();
    epb = epb.load_balancing_policy(load_balancing_policy);
    epb.ex()
}

#[rustler::nif]
fn ep_retry_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    policy: ScyllaRetryPolicy,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb = epbr.0.to_owned();
    let x = policy.r();
    epb = epb.retry_policy(x);
    epb.ex()
}

#[rustler::nif]
fn ep_speculative_execution_policy(
    epbr: ResourceArc<ExecutionProfileBuilderResource>,
    policy: ScyllaSpeculativeExecutionPolicy,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mut epb = epbr.0.to_owned();
    epb = epb.speculative_execution_policy(Option::from(policy.r()));
    epb.ex()
}

#[rustler::nif]
fn ep_build(
    epbr: ResourceArc<ExecutionProfileBuilderResource>) -> ResourceArc<ExecutionProfileResource> {
    let ep = epbr.0.to_owned().build();
    ep.ex()
}

#[rustler::nif]
fn ep_into_handle(
    epr: ResourceArc<ExecutionProfileResource>) -> ResourceArc<ExecutionProfileHandleResource> {
    let eph = epr.0.to_owned().into_handle();
    eph.ex()
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.LoadBalancingPolicy"]
pub struct ScyllaLoadBalancingPolicy {
    datacenter: String,
    rack: String,
    is_token_aware: bool,
    permit_dc_failover: bool,
    enable_shuffling_replicas: bool,
    // TODO: latency_awareness
}

impl ToRust<Arc<dyn LoadBalancingPolicy>> for ScyllaLoadBalancingPolicy {
    fn r(self) -> Arc<dyn LoadBalancingPolicy> {
        let mut lbpb = DefaultPolicy::builder();
        // print datacenter
        println!("datacenter: {:?}", self.datacenter);
        lbpb = lbpb.prefer_datacenter(self.datacenter);
        println!("rack: {:?}", self.rack);
        lbpb = lbpb.prefer_rack(self.rack);
        println!("is_token_aware: {:?}", self.is_token_aware);
        lbpb = lbpb.token_aware(self.is_token_aware);
        println!("permit_dc_failover: {:?}", self.permit_dc_failover);
        lbpb = lbpb.permit_dc_failover(self.permit_dc_failover);
        println!("enable_shuffling_replicas: {:?}", self.enable_shuffling_replicas);
        lbpb = lbpb.enable_shuffling_replicas(self.enable_shuffling_replicas);
        lbpb.build()
    }
}

#[derive(NifUnitEnum)]
pub enum ScyllaRetryPolicy {
    DefaultRetryPolicy,
    FallthroughRetryPolicy,
    DowngradingConsistencyRetryPolicy,
}

impl ToRust<Box<dyn RetryPolicy + Send + Sync>> for ScyllaRetryPolicy {
    fn r(self) -> Box<dyn RetryPolicy + Send + Sync> {
        match self {
            Self::DefaultRetryPolicy => Box::new(DefaultRetryPolicy::default()),
            Self::FallthroughRetryPolicy => Box::new(FallthroughRetryPolicy::default()),
            Self::DowngradingConsistencyRetryPolicy => Box::new(DowngradingConsistencyRetryPolicy::default()),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.SimpleSpeculativeExecutionPolicy"]
pub struct ScyllaSimpleSpeculativeExecutionPolicy {
    /// The maximum number of speculative executions that will be triggered
    /// for a given request (does not include the initial request)
    pub max_retry_count: usize,

    /// The delay between each speculative execution
    pub retry_interval_ms: u64,
}

impl ToRust<SimpleSpeculativeExecutionPolicy> for ScyllaSimpleSpeculativeExecutionPolicy {
    fn r(self) -> SimpleSpeculativeExecutionPolicy {
        SimpleSpeculativeExecutionPolicy {
            max_retry_count: self.max_retry_count,
            retry_interval: Duration::from_millis(self.retry_interval_ms),
        }
    }
}

/// A policy that triggers speculative executions when the request to the current
/// host is above a given percentile.
#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.PercentileSpeculativeExecutionPolicy"]
pub struct ScyllaPercentileSpeculativeExecutionPolicy {
    /// The maximum number of speculative executions that will be triggered
    /// for a given request (does not include the initial request)
    pub max_retry_count: usize,

    /// The percentile that a request's latency must fall into to be considered
    /// slow (ex: 99.0)
    pub percentile: f64,
}

impl ToRust<PercentileSpeculativeExecutionPolicy> for ScyllaPercentileSpeculativeExecutionPolicy {
    fn r(self) -> PercentileSpeculativeExecutionPolicy {
        PercentileSpeculativeExecutionPolicy {
            max_retry_count: self.max_retry_count,
            percentile: self.percentile,
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum ScyllaSpeculativeExecutionPolicy {
    Simple(ScyllaSimpleSpeculativeExecutionPolicy),
    Percentile(ScyllaPercentileSpeculativeExecutionPolicy),
}

impl ToRust<Arc<dyn SpeculativeExecutionPolicy>> for ScyllaSpeculativeExecutionPolicy {
    fn r(self) -> Arc<dyn SpeculativeExecutionPolicy> {
        match self {
            Self::Simple(e) => Arc::new(e.r()),
            Self::Percentile(e) => Arc::new(e.r()),
        }
    }
}
