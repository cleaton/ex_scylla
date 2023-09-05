use rustler::NifUnitEnum;
use scylla::retry_policy::{DefaultRetryPolicy, FallthroughRetryPolicy, RetryPolicy};

#[derive(NifUnitEnum)]
pub enum ScyllaRetryPolicy {
    DefaultRetryPolicy,
    FallthroughRetryPolicy,
}

impl Into<Box<dyn RetryPolicy>> for ScyllaRetryPolicy {
    fn into(self) -> Box<dyn RetryPolicy> {
        match self {
            Self::DefaultRetryPolicy => Box::new(DefaultRetryPolicy::default()),
            Self::FallthroughRetryPolicy => Box::new(FallthroughRetryPolicy::default()),
        }
    }
}
