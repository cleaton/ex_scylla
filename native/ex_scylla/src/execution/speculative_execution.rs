use std::{sync::Arc, time::Duration};

use rustler::{NifStruct, NifUntaggedEnum};
use scylla::policies::speculative_execution::{
    PercentileSpeculativeExecutionPolicy, SimpleSpeculativeExecutionPolicy,
    SpeculativeExecutionPolicy,
};

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.SimpleSpeculativeExecutionPolicy"]
pub struct ScyllaSimpleSpeculativeExecutionPolicy {
    /// The maximum number of speculative executions that will be triggered
    /// for a given request (does not include the initial request)
    pub max_retry_count: usize,

    /// The delay between each speculative execution
    pub retry_interval_ms: u64,
}

impl From<ScyllaSimpleSpeculativeExecutionPolicy> for Arc<dyn SpeculativeExecutionPolicy> {
    fn from(val: ScyllaSimpleSpeculativeExecutionPolicy) -> Self {
        Arc::new(SimpleSpeculativeExecutionPolicy {
            max_retry_count: val.max_retry_count,
            retry_interval: Duration::from_millis(val.retry_interval_ms),
        })
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

impl From<ScyllaPercentileSpeculativeExecutionPolicy> for Arc<dyn SpeculativeExecutionPolicy> {
    fn from(val: ScyllaPercentileSpeculativeExecutionPolicy) -> Self {
        Arc::new(PercentileSpeculativeExecutionPolicy {
            max_retry_count: val.max_retry_count,
            percentile: val.percentile,
        })
    }
}

#[derive(NifUntaggedEnum)]
pub enum ScyllaSpeculativeExecutionPolicy {
    Simple(ScyllaSimpleSpeculativeExecutionPolicy),
    Percentile(ScyllaPercentileSpeculativeExecutionPolicy),
}

impl From<ScyllaSpeculativeExecutionPolicy> for Arc<dyn SpeculativeExecutionPolicy> {
    fn from(val: ScyllaSpeculativeExecutionPolicy) -> Self {
        match val {
            ScyllaSpeculativeExecutionPolicy::Simple(e) => e.into(),
            ScyllaSpeculativeExecutionPolicy::Percentile(e) => e.into(),
        }
    }
}
