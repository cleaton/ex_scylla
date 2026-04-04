use std::time::Duration;

use super::runtime;
pub mod types;
use crate::batch::types::BatchResource;
use crate::execution::execution_profile_handle::ExecutionProfileHandleResource;
use crate::prepared_statement::types::*;
use crate::utils::*;
use rustler::env::{OwnedEnv, SavedTerm};
use rustler::types::atom;
use rustler::{Atom, Encoder, Env, NifResult, ResourceArc, Term};
use scylla::client::session::Session;
use scylla::response::PagingStateResponse;
use scylla::statement::batch::Batch;
use scylla::value::CqlValue;
use types::*;

fn decode_values<'a>(values: Vec<Term<'a>>) -> NifResult<Vec<CqlValue>> {
    let mut cql_values: Vec<CqlValue> = Vec::new();
    for term in values {
        let sv: ScyllaValue = term.decode()?;
        cql_values.push(sv.into());
    }
    Ok(cql_values)
}

#[rustler::nif]
fn s_await_schema_agreement<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.await_schema_agreement().await;
        match res {
            Ok(_id) => ScyllaResult::Unwrapped(atom::ok()),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_await_timed_schema_agreement<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    timeout_ms: u64,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = tokio::time::timeout(
            Duration::from_millis(timeout_ms),
            session.await_schema_agreement(),
        )
        .await;
        match res {
            Ok(Ok(_id)) => ScyllaResult::Ok(true),
            Ok(Err(e)) => ScyllaResult::Err(e.to_string()),
            Err(_) => ScyllaResult::Err("timeout".to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_batch<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    batch: ResourceArc<BatchResource>,
    values: Vec<Vec<Term<'a>>>,
) -> NifResult<Atom> {
    let mut row_values: Vec<Vec<CqlValue>> = Vec::new();
    for row_terms in values {
        row_values.push(decode_values(row_terms)?);
    }

    async_elixir!(
        env,
        opaque,
        {
            let session: &Session = &session.0;
            let batch: &Batch = &batch.0;
            session.batch(batch, row_values).await
        },
        |env, res| {
            match res {
                Ok(qr) => ScyllaResult::Ok(ScyllaQueryResult::new(env, qr)),
                Err(e) => ScyllaResult::Err(e.ex()),
            }
        }
    )
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_calculate_token<'a>(
    _session: ResourceArc<SessionResource>,
    prepared: ResourceArc<PreparedStatementResource>,
    values: Vec<Term<'a>>,
) -> NifResult<Option<ScyllaToken>> {
    let cql_values = decode_values(values)?;

    match prepared.0.calculate_token(&cql_values) {
        Ok(token) => Ok(token.map(|t| t.into())),
        Err(_) => Ok(None),
    }
}

#[rustler::nif]
pub fn s_calculate_token_for_partition_key<'a>(
    session: ResourceArc<SessionResource>,
    keyspace: String,
    table: String,
    partition_key: Vec<Term<'a>>,
) -> NifResult<Option<ScyllaToken>> {
    let cql_values = decode_values(partition_key)?;

    let session: &Session = &session.0;
    let cluster_state = session.get_cluster_state();

    match cluster_state.compute_token(&keyspace, &table, &cql_values) {
        Ok(token) => Ok(Some(token.into())),
        Err(_) => Ok(None),
    }
}

#[rustler::nif]
fn s_check_schema_agreement<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.check_schema_agreement().await;
        match res {
            Ok(agree) => ScyllaResult::Ok(agree.is_some()),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_execute<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    prepared: ResourceArc<PreparedStatementResource>,
    values: Vec<Term<'a>>,
) -> NifResult<Atom> {
    let cql_values = decode_values(values)?;

    async_elixir!(
        env,
        opaque,
        {
            let session: &Session = &session.0;
            session.execute_unpaged(&prepared.0, cql_values).await
        },
        |env, res| {
            match res {
                Ok(qr) => ScyllaResult::Ok(ScyllaQueryResult::new(env, qr)),
                Err(e) => ScyllaResult::Err(e.ex()),
            }
        }
    )
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_execute_paged<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    prepared: ResourceArc<PreparedStatementResource>,
    values: Vec<Term<'a>>,
    paging_state: Option<ScyllaPageState>,
) -> NifResult<Atom> {
    let cql_values = decode_values(values)?;
    let paging_state = paging_state
        .map(|s| s.0)
        .unwrap_or(scylla::response::PagingState::start());
    async_elixir!(
        env,
        opaque,
        {
            let session: &Session = &session.0;
            session
                .execute_single_page(&prepared.0, cql_values, paging_state)
                .await
        },
        |env, res| {
            match res {
                Ok((qr, psr)) => {
                    let mut scylla_qr = ScyllaQueryResult::new(env, qr);
                    if let PagingStateResponse::HasMorePages { state } = psr {
                        scylla_qr.paging_state =
                            state.as_bytes_slice().map(|arc| ScyllaBinary(arc.to_vec()));
                    }
                    ScyllaResult::Ok(scylla_qr)
                }
                Err(e) => ScyllaResult::Err(e.ex()),
            }
        }
    )
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_get_cluster_state(session: ResourceArc<SessionResource>) -> ScyllaClusterState {
    let session: &Session = &session.0;
    (&*session.get_cluster_state()).into()
}

#[rustler::nif]
fn s_get_default_execution_profile_handle(
    session: ResourceArc<SessionResource>,
) -> ResourceArc<ExecutionProfileHandleResource> {
    let session: &Session = &session.0;
    ResourceArc::new(ExecutionProfileHandleResource(
        session.get_default_execution_profile_handle().clone(),
    ))
}

#[rustler::nif]
fn s_prepare_batch<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    batch: ResourceArc<BatchResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let batch: &Batch = &batch.0;
        let res = session.prepare_batch(batch).await;
        match res {
            Ok(b) => ScyllaResult::Ok(ResourceArc::new(BatchResource(b))),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_get_keyspace(session: ResourceArc<SessionResource>) -> Option<String> {
    let session: &Session = &session.0;
    session.get_keyspace().map(|s| s.to_string())
}

#[rustler::nif]
fn s_get_metrics(session: ResourceArc<SessionResource>) -> ScyllaMetrics {
    let session: &Session = &session.0;
    (&*session.get_metrics()).into()
}

#[rustler::nif]
fn s_get_tracing_info<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    tracing_id: ScyllaBinary,
) -> NifResult<Atom> {
    let tracing_id =
        uuid::Uuid::from_slice(&tracing_id.0).map_err(|_| rustler::Error::Atom("invalid_uuid"))?;
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.get_tracing_info(&tracing_id).await;
        match res {
            Ok(ti) => ScyllaResult::Ok(ScyllaTracingInfo::from(ti)),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_fetch_schema_version<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.await_schema_agreement().await;
        match res {
            Ok(id) => ScyllaResult::Ok(ScyllaBinary(id.as_bytes().to_vec())),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_prepare<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    query: ScyllaQuery,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.prepare(query).await;
        match res {
            Ok(ps) => ScyllaResult::Ok(ResourceArc::new(PreparedStatementResource(ps))),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_query<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    query: ScyllaQuery,
    values: Vec<Term<'a>>,
) -> NifResult<Atom> {
    let cql_values = decode_values(values)?;
    async_elixir!(
        env,
        opaque,
        {
            let session: &Session = &session.0;
            session.query_unpaged(query, cql_values).await
        },
        |env, res| {
            match res {
                Ok(qr) => ScyllaResult::Ok(ScyllaQueryResult::new(env, qr)),
                Err(e) => ScyllaResult::Err(e.ex()),
            }
        }
    )
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_query_paged<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    query: ScyllaQuery,
    values: Vec<Term<'a>>,
    paging_state: Option<ScyllaPageState>,
) -> NifResult<Atom> {
    let cql_values = decode_values(values)?;
    let paging_state = paging_state
        .map(|s| s.0)
        .unwrap_or(scylla::response::PagingState::start());
    async_elixir!(
        env,
        opaque,
        {
            let session: &Session = &session.0;
            session
                .query_single_page(query, cql_values, paging_state)
                .await
        },
        |env, res| {
            match res {
                Ok((qr, psr)) => {
                    let mut scylla_qr = ScyllaQueryResult::new(env, qr);
                    if let PagingStateResponse::HasMorePages { state } = psr {
                        scylla_qr.paging_state =
                            state.as_bytes_slice().map(|arc| ScyllaBinary(arc.to_vec()));
                    }
                    ScyllaResult::Ok(scylla_qr)
                }
                Err(e) => ScyllaResult::Err(e.ex()),
            }
        }
    )
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_refresh_metadata<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.refresh_metadata().await;
        match res {
            Ok(_) => ScyllaResult::Unwrapped(atom::ok()),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}

#[rustler::nif]
fn s_use_keyspace<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    keyspace_name: String,
    case_sensitive: bool,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.use_keyspace(keyspace_name, case_sensitive).await;
        match res {
            Ok(_) => ScyllaResult::Unwrapped(atom::ok()),
            Err(e) => ScyllaResult::Err(e.to_string()),
        }
    })
    .map(|_| atom::ok())
}
