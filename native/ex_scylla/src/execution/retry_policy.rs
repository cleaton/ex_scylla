use rustler::NifUnitEnum;
use scylla::policies::retry::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};
use std::sync::Arc;

#[derive(NifUnitEnum)]
pub enum ScyllaRetryPolicy {
    DefaultRetryPolicy,
    FallthroughRetryPolicy,
}

impl From<ScyllaRetryPolicy> for Arc<dyn RetryPolicy> {
    fn from(val: ScyllaRetryPolicy) -> Self {
        match val {
            ScyllaRetryPolicy::DefaultRetryPolicy => Arc::new(DefaultRetryPolicy),
            ScyllaRetryPolicy::FallthroughRetryPolicy => Arc::new(FallthroughRetryPolicy),
        }
    }
}
