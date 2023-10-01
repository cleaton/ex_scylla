use rustler::Atom;
use rustler::ResourceArc;
use scylla::batch::{Batch, BatchStatement};

pub mod types;
use crate::consts::*;
use crate::execution::execution_profile_handle::ExecutionProfileHandleResource;
use crate::session::types::ScyllaBatchStatement;
use crate::types::*;
use types::*;

#[rustler::nif]
fn b_append_statement(
    batch: ResourceArc<BatchResource>,
    statement: ScyllaBatchStatement,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.append_statement(statement);
    ResourceArc::new(BatchResource(b))
}

#[rustler::nif]
fn b_get_execution_profile_handle(qr: ResourceArc<BatchResource>) -> Option<ResourceArc<ExecutionProfileHandleResource>> {
    let b: &Batch = &qr.0;
    b.get_execution_profile_handle().map(|h| ResourceArc::new(ExecutionProfileHandleResource(h.clone())))
}

#[rustler::nif]
fn b_set_execution_profile_handle(q: ResourceArc<BatchResource>, profile_handle: Option<ResourceArc<ExecutionProfileHandleResource>>) -> ResourceArc<BatchResource> {
    let mut b: Batch = q.0.to_owned();
    b.set_execution_profile_handle(profile_handle.map(|ephr| ephr.0.clone()));
    ResourceArc::new(BatchResource(b))
}

#[rustler::nif]
fn b_get_consistency(batch: ResourceArc<BatchResource>) -> Option<ScyllaConsistency> {
    let b: &Batch = &batch.0;
    b.get_consistency().map(|c| c.into())
}
#[rustler::nif]
fn b_get_is_idempotent(batch: ResourceArc<BatchResource>) -> bool {
    let b: &Batch = &batch.0;
    b.get_is_idempotent()
}
#[rustler::nif]
fn b_get_retry_policy(_batch: ResourceArc<BatchResource>) -> Atom {
    not_implemented_yet()
}
#[rustler::nif]
fn b_get_serial_consistency(batch: ResourceArc<BatchResource>) -> Option<ScyllaSerialConsistency> {
    let b: &Batch = &batch.0;
    b.get_serial_consistency().map(|c| c.into())
}
#[rustler::nif]
fn b_get_timestamp(batch: ResourceArc<BatchResource>) -> Option<i64> {
    let b: &Batch = &batch.0;
    b.get_timestamp()
}
#[rustler::nif]
fn b_get_tracing(batch: ResourceArc<BatchResource>) -> bool {
    let b: &Batch = &batch.0;
    b.get_tracing()
}
#[rustler::nif]
fn b_get_type(batch: ResourceArc<BatchResource>) -> ScyllaBatchType {
    let b: &Batch = &batch.0;
    b.get_type().into()
}

#[rustler::nif]
fn b_new(batch_type: ScyllaBatchType) -> ResourceArc<BatchResource> {
    ResourceArc::new(BatchResource(Batch::new(batch_type.into())))
}
#[rustler::nif]
fn b_new_with_statements(
    batch_type: ScyllaBatchType,
    statements: Vec<ScyllaBatchStatement>,
) -> ResourceArc<BatchResource> {
    let statements: Vec<BatchStatement> = statements.into_iter().map(|s| s.into()).collect();
    ResourceArc::new(BatchResource(Batch::new_with_statements(
        batch_type.into(),
        statements,
    )))
}
#[rustler::nif]
fn b_set_consistency(
    batch: ResourceArc<BatchResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.set_consistency(consistency.into());
    ResourceArc::new(BatchResource(b))
}
#[rustler::nif]
fn b_set_is_idempotent(
    batch: ResourceArc<BatchResource>,
    is_idempotent: bool,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.set_is_idempotent(is_idempotent);
    ResourceArc::new(BatchResource(b))
}
#[rustler::nif]
fn b_set_retry_policy(_batch: ResourceArc<BatchResource>, _retry_policy: bool) -> Atom {
    not_implemented_yet()
}
#[rustler::nif]
fn b_set_serial_consistency(
    batch: ResourceArc<BatchResource>,
    sc: Option<ScyllaSerialConsistency>,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.set_serial_consistency(sc.map(|sc| sc.into()));
    ResourceArc::new(BatchResource(b))
}
#[rustler::nif]
fn b_set_timestamp(
    batch: ResourceArc<BatchResource>,
    timestamp_micros: Option<i64>,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.set_timestamp(timestamp_micros);
    ResourceArc::new(BatchResource(b))
}
#[rustler::nif]
fn b_set_tracing(
    batch: ResourceArc<BatchResource>,
    should_trace: bool,
) -> ResourceArc<BatchResource> {
    let mut b: Batch = batch.0.to_owned();
    b.set_tracing(should_trace);
    ResourceArc::new(BatchResource(b))
}
