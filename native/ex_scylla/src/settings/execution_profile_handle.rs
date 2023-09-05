use rustler::ResourceArc;
use scylla::execution_profile::ExecutionProfileHandle;
use std::cell::Cell;
use std::sync::Mutex;

use super::execution_profile::ExecutionProfileResource;
use super::execution_profile_builder::ExecutionProfileBuilderResource;
pub struct ExecutionProfileHandleResource(pub Mutex<Cell<ExecutionProfileHandle>>);

fn eph_map_to_another_profile(
    ephr: ResourceArc<ExecutionProfileHandleResource>,
    profile: ResourceArc<ExecutionProfileResource>,
) {
    let mutex: &Mutex<Cell<ExecutionProfileHandle>> = &ephr.0;
    let mut guard = mutex.lock().unwrap();
    guard.get_mut().map_to_another_profile(profile.0.clone());
}

fn eph_pointee_to_builder(
    ephr: ResourceArc<ExecutionProfileHandleResource>,
) -> ResourceArc<ExecutionProfileBuilderResource> {
    let mutex: &Mutex<Cell<ExecutionProfileHandle>> = &ephr.0;
    let mut guard = mutex.lock().unwrap();
    ResourceArc::new(ExecutionProfileBuilderResource(Mutex::new(Cell::new(
        guard.get_mut().pointee_to_builder(),
    ))))
}
