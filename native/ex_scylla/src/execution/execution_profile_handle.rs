use rustler::ResourceArc;
use scylla::execution_profile::ExecutionProfileHandle;
use std::cell::Cell;
use std::sync::Mutex;
use std::panic::RefUnwindSafe;

use super::execution_profile::ExecutionProfileResource;
use super::execution_profile_builder::ExecutionProfileBuilderResource;
pub struct ExecutionProfileHandleResource(pub ExecutionProfileHandle);

impl RefUnwindSafe for ExecutionProfileHandleResource {}

#[rustler::nif]
fn eph_map_to_another_profile(
    ephr: ResourceArc<ExecutionProfileHandleResource>,
    profile: ResourceArc<ExecutionProfileResource>,
) {
    let mut eph: ExecutionProfileHandle = ephr.0.clone();
    eph.map_to_another_profile(profile.0.lock().unwrap().clone())
}

#[rustler::nif]
fn eph_pointee_to_builder(
    ephr: ResourceArc<ExecutionProfileHandleResource>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let eph: ExecutionProfileHandle = ephr.0.clone();
    ResourceArc::new(ExecutionProfileBuilderResource(Mutex::new(Cell::new(
        eph.pointee_to_builder()
    ))))
}
