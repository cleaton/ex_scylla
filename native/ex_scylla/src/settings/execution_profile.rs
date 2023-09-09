use std::{cell::Cell, sync::Mutex};

use rustler::ResourceArc;
use scylla::ExecutionProfile;

use super::{
    execution_profile_builder::ExecutionProfileBuilderResource,
    execution_profile_handle::ExecutionProfileHandleResource,
};

pub struct ExecutionProfileResource(pub ExecutionProfile);

#[rustler::nif]
pub fn ep_builder() -> ResourceArc<ExecutionProfileBuilderResource> {
    ResourceArc::new(ExecutionProfileBuilderResource(Mutex::new(Cell::new(
        ExecutionProfile::builder(),
    ))))
}

#[rustler::nif]
pub fn ep_into_handle(
    ep: ResourceArc<ExecutionProfileResource>,
) -> ResourceArc<ExecutionProfileHandleResource> {
    let profile: ExecutionProfile = ep.0.clone();
    ResourceArc::new(ExecutionProfileHandleResource(Mutex::new(Cell::new(
        profile.into_handle(),
    ))))
}

#[rustler::nif]
pub fn ep_into_handle_with_label(
    ep: ResourceArc<ExecutionProfileResource>,
    label: String,
) -> ResourceArc<ExecutionProfileHandleResource> {
    let profile: ExecutionProfile = ep.0.clone();
    ResourceArc::new(ExecutionProfileHandleResource(Mutex::new(Cell::new(
        profile.into_handle_with_label(label),
    ))))
}

#[rustler::nif]
pub fn ep_to_builder(
    ep: ResourceArc<ExecutionProfileResource>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    ResourceArc::new(ExecutionProfileBuilderResource(Mutex::new(Cell::new(
        ep.0.to_builder(),
    ))))
}
