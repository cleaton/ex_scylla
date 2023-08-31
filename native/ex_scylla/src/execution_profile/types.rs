use rustler::ResourceArc;
use scylla::execution_profile::{ExecutionProfileBuilder, ExecutionProfileHandle};
use scylla::ExecutionProfile;

use crate::utils::*;

pub struct ExecutionProfileBuilderResource(pub ExecutionProfileBuilder);

to_elixir!(ExecutionProfileBuilder, ResourceArc<ExecutionProfileBuilderResource>, |epb | {
        ResourceArc::new(ExecutionProfileBuilderResource(epb))
    }
);

pub struct ExecutionProfileResource(pub ExecutionProfile);

to_elixir!(ExecutionProfile, ResourceArc<ExecutionProfileResource>, |ep| {
    ResourceArc::new(ExecutionProfileResource(ep))
});

pub struct ExecutionProfileHandleResource(pub ExecutionProfileHandle);

to_elixir!(ExecutionProfileHandle, ResourceArc<ExecutionProfileHandleResource>, |eph| {
    ResourceArc::new(ExecutionProfileHandleResource(eph))
});
