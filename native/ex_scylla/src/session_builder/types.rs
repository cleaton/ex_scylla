use rustler::NifTaggedEnum;
use rustler::{NifStruct, NifUnitEnum, NifUntaggedEnum};
use scylla::load_balancing::{
    DcAwareRoundRobinPolicy, LoadBalancingPolicy, RoundRobinPolicy, TokenAwarePolicy,
};
use scylla::retry_policy::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};
use scylla::speculative_execution::{
    PercentileSpeculativeExecutionPolicy, SimpleSpeculativeExecutionPolicy,
};
use scylla::transport::session::PoolSize;
use scylla::transport::speculative_execution::SpeculativeExecutionPolicy;
use scylla::SessionBuilder;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::session::types::ScyllaIpAddr;
//use scylla::transport::retry_policy::RetryPolicy;
use crate::utils::*;
use std::cell::Cell;

pub struct SessionBuilderResource(pub Mutex<Cell<SessionBuilder>>);

impl Deref for SessionBuilderResource {
    type Target = Mutex<Cell<SessionBuilder>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//clone_enum!(RetryPolicy, ScyllaRetryPolicy, {DefaultRetry, FallthroughRetry});
#[derive(NifUnitEnum)]
pub enum ScyllaRetryPolicy {
    DefaultRetryPolicy,
    FallthroughRetryPolicy,
}

impl ToRust<Box<dyn RetryPolicy + Send + Sync>> for ScyllaRetryPolicy {
    fn r(self) -> Box<dyn RetryPolicy + Send + Sync> {
        match self {
            Self::DefaultRetryPolicy => Box::new(DefaultRetryPolicy::default()),
            Self::FallthroughRetryPolicy => Box::new(FallthroughRetryPolicy::default()),
        }
    }
}

#[derive(NifTaggedEnum)]
pub enum ScyllaPoolSize {
    PerHost(usize),
    PerShard(usize),
}

impl ToRust<PoolSize> for ScyllaPoolSize {
    fn r(self) -> PoolSize {
        match self {
            Self::PerHost(v) => {
                PoolSize::PerHost(v.try_into().expect("invalid per-host pool size"))
            }
            Self::PerShard(v) => {
                PoolSize::PerShard(v.try_into().expect("invalid per-shard pool size"))
            }
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

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.DcAwareRoundRobinPolicy"]
pub struct ScyllaDcAwareRoundRobinPolicy {
    local_dc: String,
    token_aware: bool,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.RoundRobinPolicy"]
pub struct ScyllaRoundRobinPolicy {
    token_aware: bool,
}

#[derive(NifUntaggedEnum)]
pub enum ScyllaLoadBalancingPolicy {
    DcAwareRoundRobin(ScyllaDcAwareRoundRobinPolicy),
    RoundRobin(ScyllaRoundRobinPolicy),
}

impl ToRust<Arc<dyn LoadBalancingPolicy>> for ScyllaLoadBalancingPolicy {
    fn r(self) -> Arc<dyn LoadBalancingPolicy> {
        match self {
            Self::DcAwareRoundRobin(dwrr) => {
                let p = DcAwareRoundRobinPolicy::new(dwrr.local_dc);
                match dwrr.token_aware {
                    true => Arc::new(TokenAwarePolicy::new(Box::new(p))),
                    false => Arc::new(p),
                }
            }
            Self::RoundRobin(rr) => {
                let p = RoundRobinPolicy::new();
                match rr.token_aware {
                    true => Arc::new(TokenAwarePolicy::new(Box::new(p))),
                    false => Arc::new(p),
                }
            }
        }
    }
}

pub type ScyllaSocketAddr = (ScyllaIpAddr, u16);

impl ToRust<SocketAddr> for ScyllaSocketAddr {
    fn r(self) -> SocketAddr {
        let (sia, port) = self;
        SocketAddr::new(sia.into(), port)
    }
}
