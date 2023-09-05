use std::{sync::Arc, time::Duration};

use rustler::{NifStruct, NifUntaggedEnum};
use scylla::speculative_execution::{
    PercentileSpeculativeExecutionPolicy, SimpleSpeculativeExecutionPolicy,
    SpeculativeExecutionPolicy,
};

use crate::utils::ToRust;

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.SimpleSpeculativeExecutionPolicy"]
pub struct ScyllaSimpleSpeculativeExecutionPolicy {
    /// The maximum number of speculative executions that will be triggered
    /// for a given request (does not include the initial request)
    pub max_retry_count: usize,

    /// The delay between each speculative execution
    pub retry_interval_ms: u64,
}

impl Into<Arc<dyn SpeculativeExecutionPolicy>> for ScyllaSimpleSpeculativeExecutionPolicy {
    fn into(self) -> Arc<dyn SpeculativeExecutionPolicy> {
        Arc::new(SimpleSpeculativeExecutionPolicy {
            max_retry_count: self.max_retry_count,
            retry_interval: Duration::from_millis(self.retry_interval_ms),
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

impl Into<Arc<dyn SpeculativeExecutionPolicy>> for ScyllaPercentileSpeculativeExecutionPolicy {
    fn into(self) -> Arc<dyn SpeculativeExecutionPolicy> {
        Arc::new(PercentileSpeculativeExecutionPolicy {
            max_retry_count: self.max_retry_count,
            percentile: self.percentile,
        })
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
            Self::Simple(e) => e.into(),
            Self::Percentile(e) => e.into(),
        }
    }
}
