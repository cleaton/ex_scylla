use rustler::NifUnitEnum;
use scylla::policies::retry::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};
use std::sync::Arc;

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
