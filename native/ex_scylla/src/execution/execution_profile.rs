use std::{cell::Cell, sync::Mutex, panic::RefUnwindSafe};

use rustler::ResourceArc;
use scylla::ExecutionProfile;

use super::{
    execution_profile_builder::ExecutionProfileBuilderResource,
    execution_profile_handle::ExecutionProfileHandleResource,
};

pub struct ExecutionProfileResource(pub Mutex<ExecutionProfile>);

impl RefUnwindSafe for ExecutionProfileResource {}

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
    let profile = ep.0.lock().unwrap().clone();
    ResourceArc::new(ExecutionProfileHandleResource(
        profile.into_handle()
    ))
}

#[rustler::nif]
pub fn ep_into_handle_with_label(
    ep: ResourceArc<ExecutionProfileResource>,
    label: String,
) -> ResourceArc<ExecutionProfileHandleResource> {
    let profile = ep.0.lock().unwrap().clone();
    ResourceArc::new(ExecutionProfileHandleResource(
        profile.into_handle_with_label(label)
    ))
}

#[rustler::nif]
pub fn ep_to_builder(
    ep: ResourceArc<ExecutionProfileResource>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    ResourceArc::new(ExecutionProfileBuilderResource(Mutex::new(Cell::new(
        ep.0.lock().unwrap().to_builder(),
    ))))
}
