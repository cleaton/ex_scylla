pub mod types;
use std::time::Duration;

use crate::consts::*;
use crate::execution::execution_profile_handle::ExecutionProfileHandleResource;
use crate::types::*;
use crate::utils::*;
use rustler::{Atom, ResourceArc};
use scylla::query::Query;
use types::*;

#[rustler::nif]
fn q_get_execution_profile_handle(qr: ResourceArc<QueryResource>) -> Option<ResourceArc<ExecutionProfileHandleResource>> {
    let q: &Query = &qr.0;
    q.get_execution_profile_handle().map(|h| ResourceArc::new(ExecutionProfileHandleResource(h.clone())))
}

#[rustler::nif]
fn q_get_request_timeout(qr: ResourceArc<QueryResource>) -> Option<u64> {
    let q: &Query = &qr.0;
    q.get_request_timeout().map(|d| d.as_millis() as u64)
}

#[rustler::nif]
fn q_set_execution_profile_handle(q: ResourceArc<QueryResource>, profile_handle: Option<ResourceArc<ExecutionProfileHandleResource>>) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_execution_profile_handle(profile_handle.map(|ephr| ephr.0.clone()));
    q.ex()
}

#[rustler::nif]
fn q_set_request_timeout(q: ResourceArc<QueryResource>, timeout_ms: Option<u64>) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_request_timeout(timeout_ms.map(|ms| Duration::from_millis(ms)));
    q.ex()
}

#[rustler::nif]
fn q_disable_paging(q: ResourceArc<QueryResource>) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.disable_paging();
    q.ex()
}

#[rustler::nif]
fn q_get_consistency(q: ResourceArc<QueryResource>) -> Option<ScyllaConsistency> {
    let q: &Query = &q.0;
    q.get_consistency().map(|c| c.into())
}

#[rustler::nif]
fn q_get_is_idempotent(q: ResourceArc<QueryResource>) -> bool {
    let q: &Query = &q.0;
    q.get_is_idempotent()
}

#[rustler::nif]
fn q_get_page_size(q: ResourceArc<QueryResource>) -> Option<i32> {
    let q: &Query = &q.0;
    q.get_page_size()
}

#[rustler::nif]
fn q_get_retry_policy(_q: ResourceArc<QueryResource>) -> Atom {
    not_implemented_yet()
}

#[rustler::nif]
fn q_get_serial_consistency(q: ResourceArc<QueryResource>) -> Option<ScyllaSerialConsistency> {
    let q: &Query = &q.0;
    q.get_serial_consistency().map(|c| c.into())
}

#[rustler::nif]
fn q_get_timestamp(q: ResourceArc<QueryResource>) -> Option<i64> {
    let q: &Query = &q.0;
    q.get_timestamp()
}

#[rustler::nif]
fn q_get_tracing(q: ResourceArc<QueryResource>) -> bool {
    let q: &Query = &q.0;
    q.get_tracing()
}

#[rustler::nif]
fn q_new(query_text: String) -> ResourceArc<QueryResource> {
    Query::new(query_text).ex()
}

#[rustler::nif]
fn q_set_consistency(
    q: ResourceArc<QueryResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_consistency(consistency.into());
    q.ex()
}

#[rustler::nif]
fn q_set_is_idempotent(
    q: ResourceArc<QueryResource>,
    is_idempotent: bool,
) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_is_idempotent(is_idempotent);
    q.ex()
}

#[rustler::nif]
fn q_set_page_size(q: ResourceArc<QueryResource>, page_size: i32) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_page_size(page_size);
    q.ex()
}

#[rustler::nif]
fn q_set_retry_policy(_q: ResourceArc<QueryResource>) -> Atom {
    not_implemented_yet()
}

#[rustler::nif]
fn q_set_serial_consistency(
    q: ResourceArc<QueryResource>,
    sc: Option<ScyllaSerialConsistency>,
) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_serial_consistency(sc.map(|sc| sc.into()));
    q.ex()
}

#[rustler::nif]
fn q_set_timestamp(
    q: ResourceArc<QueryResource>,
    timestamp: Option<i64>,
) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_timestamp(timestamp);
    q.ex()
}

#[rustler::nif]
fn q_set_tracing(q: ResourceArc<QueryResource>, should_trace: bool) -> ResourceArc<QueryResource> {
    let mut q: Query = q.0.to_owned();
    q.set_tracing(should_trace);
    q.ex()
}

#[rustler::nif]
fn q_with_page_size(q: ResourceArc<QueryResource>, page_size: i32) -> ResourceArc<QueryResource> {
    let q: Query = q.0.to_owned();
    q.with_page_size(page_size).ex()
}
