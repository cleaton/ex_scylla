use rustler::Atom;
use rustler::Error;
use rustler::NifResult;
use rustler::ResourceArc;
use scylla::frame::value::ValueList;
use scylla::prepared_statement::PreparedStatement;

pub mod types;
use crate::consts::*;
use crate::session::types::*;
use crate::types::*;
use crate::utils::*;
use types::*;

#[rustler::nif]
fn ps_compute_partition_key(
    ps: ResourceArc<PreparedStatementResource>,
    bound_values: Vec<ScyllaValue>,
) -> NifResult<ScyllaBinary> {
    let ps: &PreparedStatement = &ps.0;
    let bound_values = bound_values.r()?;
    let bound_values = bound_values
        .serialized()
        .map_err(|sve| Error::Term(Box::new(sve.ex())))?;
    ps.compute_partition_key(&bound_values)
        .map(|b| b.into())
        .map_err(|e| Error::Term(Box::new(e.ex())))
}

#[rustler::nif]
fn ps_disable_paging(
    ps: ResourceArc<PreparedStatementResource>,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.disable_paging();
    ps.ex()
}

#[rustler::nif]
fn ps_get_consistency(ps: ResourceArc<PreparedStatementResource>) -> Option<ScyllaConsistency> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_consistency().map(|c| c.into())
}

#[rustler::nif]
fn ps_get_id(ps: ResourceArc<PreparedStatementResource>) -> ScyllaBinary {
    let ps: &PreparedStatement = &ps.0;
    ps.get_id().to_owned().into()
}

#[rustler::nif]
fn ps_get_is_idempotent(ps: ResourceArc<PreparedStatementResource>) -> bool {
    let ps: &PreparedStatement = &ps.0;
    ps.get_is_idempotent()
}

#[rustler::nif]
fn ps_get_keyspace_name(ps: ResourceArc<PreparedStatementResource>) -> Option<String> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_keyspace_name().map(|s| s.to_string())
}

#[rustler::nif]
fn ps_get_page_size(ps: ResourceArc<PreparedStatementResource>) -> Option<i32> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_page_size()
}

#[rustler::nif]
fn ps_get_prepare_tracing_ids(ps: ResourceArc<PreparedStatementResource>) -> Vec<ScyllaBinary> {
    let ps: &PreparedStatement = &ps.0;
    let mut vec: Vec<ScyllaBinary> = Vec::new();
    for uuid in ps.get_prepare_tracing_ids() {
        vec.push(uuid.to_owned().into());
    }
    vec
}

#[rustler::nif]
fn ps_get_prepared_metadata(ps: ResourceArc<PreparedStatementResource>) -> ScyllaPreparedMetadata {
    let ps: &PreparedStatement = &ps.0;
    ps.get_prepared_metadata().into()
}

#[rustler::nif]
fn ps_get_retry_policy(_ps: ResourceArc<PreparedStatementResource>) -> Atom {
    not_implemented_yet()
}

#[rustler::nif]
fn ps_get_serial_consistency(
    ps: ResourceArc<PreparedStatementResource>,
) -> Option<ScyllaSerialConsistency> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_serial_consistency().map(|c| c.into())
}

#[rustler::nif]
fn ps_get_statement(ps: ResourceArc<PreparedStatementResource>) -> String {
    let ps: &PreparedStatement = &ps.0;
    ps.get_statement().to_string()
}

#[rustler::nif]
fn ps_get_table_name(ps: ResourceArc<PreparedStatementResource>) -> Option<String> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_table_name().map(|tn| tn.to_string())
}

#[rustler::nif]
fn ps_get_timestamp(ps: ResourceArc<PreparedStatementResource>) -> Option<i64> {
    let ps: &PreparedStatement = &ps.0;
    ps.get_timestamp()
}

#[rustler::nif]
fn ps_get_tracing(ps: ResourceArc<PreparedStatementResource>) -> bool {
    let ps: &PreparedStatement = &ps.0;
    ps.get_tracing()
}

#[rustler::nif]
fn ps_is_token_aware(ps: ResourceArc<PreparedStatementResource>) -> bool {
    let ps: &PreparedStatement = &ps.0;
    ps.is_token_aware()
}

#[rustler::nif]
fn ps_set_consistency(
    ps: ResourceArc<PreparedStatementResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_consistency(consistency.into());
    ps.ex()
}

#[rustler::nif]
fn ps_set_is_idempotent(
    ps: ResourceArc<PreparedStatementResource>,
    is_idempotent: bool,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_is_idempotent(is_idempotent);
    ps.ex()
}

#[rustler::nif]
fn ps_set_page_size(
    ps: ResourceArc<PreparedStatementResource>,
    page_size: i32,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_page_size(page_size);
    ps.ex()
}

#[rustler::nif]
fn ps_set_retry_policy(_ps: ResourceArc<PreparedStatementResource>) -> Atom {
    not_implemented_yet()
}

#[rustler::nif]
fn ps_set_serial_consistency(
    ps: ResourceArc<PreparedStatementResource>,
    sc: Option<ScyllaSerialConsistency>,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_serial_consistency(sc.map(|sc| sc.into()));
    ps.ex()
}

#[rustler::nif]
fn ps_set_timestamp(
    ps: ResourceArc<PreparedStatementResource>,
    timestamp_micros: Option<i64>,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_timestamp(timestamp_micros);
    ps.ex()
}

#[rustler::nif]
fn ps_set_tracing(
    ps: ResourceArc<PreparedStatementResource>,
    should_trace: bool,
) -> ResourceArc<PreparedStatementResource> {
    let mut ps: PreparedStatement = ps.0.to_owned();
    ps.set_tracing(should_trace);
    ps.ex()
}
