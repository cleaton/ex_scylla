use std::sync::Arc;
use rustler::NifUnitEnum;
use scylla::retry_policy::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};

#[derive(NifUnitEnum)]
pub enum ScyllaRetryPolicy {
    DefaultRetryPolicy,
    FallthroughRetryPolicy,
}

impl Into<Arc<dyn RetryPolicy>> for ScyllaRetryPolicy {
    fn into(self) -> Arc<dyn RetryPolicy> {
        match self {
            Self::DefaultRetryPolicy => Arc::new(DefaultRetryPolicy::default()),
            Self::FallthroughRetryPolicy => Arc::new(FallthroughRetryPolicy::default()),
        }
    }
}
