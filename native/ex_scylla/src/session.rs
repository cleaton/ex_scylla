use std::convert::TryInto;
use std::time::Duration;

use super::runtime;
pub mod types;
use crate::batch::types::BatchResource;
use crate::prepared_statement::types::*;
use crate::utils::*;
use crate::consts;
use rustler::env::{OwnedEnv, SavedTerm};
use rustler::types::atom;
use rustler::{Atom, Encoder, Env, Error, NifResult, ResourceArc, Term};
use scylla::frame::response::result::CqlValue;
use scylla::statement::batch::Batch;
use scylla::Session;
use tokio::time::Instant;
use types::*;

#[rustler::nif]
fn s_await_schema_agreement<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.await_schema_agreement().await;
        res.ex()
    });
    Ok(atom::ok())
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
        let res = session
            .await_timed_schema_agreement(Duration::from_millis(timeout_ms))
            .await;
        res.map_err(|e| e.ex())
    });
    Ok(atom::ok())
}

#[rustler::nif]
fn s_batch<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    batch: ResourceArc<BatchResource>,
    values: Vec<Vec<ScyllaValue>>,
) -> NifResult<Atom> {
    let cql_values = values
        .into_iter()
        .map(|svv| {
            svv.into_iter()
                .map(|sv| sv.try_into().map_err(|e| Error::from(e)))
                .collect::<Result<Vec<CqlValue>, Error>>()
        })
        .collect::<Result<Vec<Vec<CqlValue>>, Error>>()?;
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let batch: &Batch = &batch.0;
        let res = session.batch(batch, cql_values).await;
        res.ex()
    });
    Ok(atom::ok())
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
        res.map_err(|e| e.ex())
    });
    Ok(atom::ok())
}

#[rustler::nif]
fn s_execute<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    prepared: ResourceArc<PreparedStatementResource>,
    values: Vec<(Term<'a>, Term<'a>)>,
) -> NifResult<Atom> {
        let (_, key) = values.first().unwrap();
        let str: String = key.decode().unwrap();

        let pid = env.pid();
        let mut owned_env = OwnedEnv::new();
        //let opaque = owned_env
        //    .run(|env| -> NifResult<SavedTerm> { Ok(owned_env.save(opaque.in_env(env))) })?;
        runtime::rt().spawn(async move {
            
            let session: &Session = &session.0;
            //let values = &vec![String::from("hello")];
            let start = Instant::now();
            let _res = session.execute(&prepared.0, (str,)).await;
            let duration = start.elapsed();
            //println!("Time elapsed in execute rust is: {:?}", duration);
            let res = (atom::ok(), atom::ok());
            owned_env.send_and_clear(&pid, |env| ((consts::execute(), atom::ok()), res).encode(env));
            let duration = start.elapsed();
            //println!("Time elapsed in execute rust is 2: {:?}", duration);
        });
    Ok(atom::ok())



    //let values = values.r()?;
    //async_elixir!(env, opaque, {
    //    let session: &Session = &session.0;
    //    let res = session.execute(&prepared.0, values).await;
    //    res.ex()
    //});
    //Ok(atom::ok())
}

//s_execute_iter(_session, _prepared, _values), do: e()
// does it make sense to expose iterator as a resourcearc?

#[rustler::nif]
fn s_execute_paged<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    prepared: ResourceArc<PreparedStatementResource>,
    values: Vec<ScyllaValue>,
    paging_state: Option<ScyllaPageState>,
) -> NifResult<Atom> {
    let values = values.r()?;
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session
            .execute_paged(&prepared.0, values, paging_state.r())
            .await;
        res.ex()
    });
    Ok(atom::ok())
}

#[rustler::nif]
fn s_fetch_schema_version<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
) -> NifResult<Atom> {
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.fetch_schema_version().await;
        res.ex()
    });
    Ok(atom::ok())
}

// TODO: define clusterdata types
//s_get_cluster_data(), do: e()
//#[rustler::nif]
//fn s_get_cluster_data<'a>(
//    env: Env<'a>,
//    opaque: Term<'a>,
//    session: ResourceArc<SessionResource>,
//) -> NifResult<Atom> {
//    let session: &Session = &session.0;
//    session.get_cluster_data();
//    Ok(atom::ok())
//}

// TODO: define metrics types
//s_get_metrics(), do: e()
//#[rustler::nif]
//fn s_get_metrics<'a>(
//    env: Env<'a>,
//    opaque: Term<'a>,
//    session: ResourceArc<SessionResource>,
//) -> NifResult<Atom> {
//    let session: &Session = &session.0;
//    session.get_metrics()
//}

// TODO: define tracing types
//s_get_tracing_info(), do: e()
//#[rustler::nif]
//fn s_get_tracing_info<'a>(
//    env: Env<'a>,
//    opaque: Term<'a>,
//    session: ResourceArc<SessionResource>,
//) -> NifResult<Atom> {
//    async_elixir!(env, opaque, {
//        let session: &Session = &session.0;
//        let res = session.get_tracing_info().await;
//        res.ex()
//    });
//    Ok(atom::ok())
//}

// TODO: define tracing types & GetTracingConfig types
//s_get_tracing_info_custom(), do: e()
//fn s_get_tracing_info_custom<'a>(
//    env: Env<'a>,
//    opaque: Term<'a>,
//    session: ResourceArc<SessionResource>,
//) -> NifResult<Atom> {
//    async_elixir!(env, opaque, {
//        let session: &Session = &session.0;
//        let res = session.get_tracing_info_custom().await;
//        res.ex()
//    });
//    Ok(atom::ok())
//}

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
        res.ex()
    });
    Ok(atom::ok())
}

#[rustler::nif]
fn s_query<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    query: ScyllaQuery,
    values: Vec<ScyllaValue>,
) -> NifResult<Atom> {
    let values = values.r()?;
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.query(query, values).await;
        res.ex()
    });
    Ok(atom::ok())
}

//s_query_iter(), do: e()
// does it make sense to expose query_iterator as a resourcearc?
//#[rustler::nif]
//fn s_query_iter<'a>(
//    env: Env<'a>,
//    opaque: Term<'a>,
//    session: ResourceArc<SessionResource>,
//    query: String,
//    values: Vec<ScyllaValue>,
//) -> NifResult<Atom> {
//    let values = values.r()?;
//    async_elixir!(env, opaque, {
//        let session: &Session = &session.0;
//        let res = session.query_iter(query, values, paging_state.r()).await;
//        res.ex()
//    });
//    Ok(atom::ok())
//}

#[rustler::nif]
fn s_query_paged<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    session: ResourceArc<SessionResource>,
    query: ScyllaQuery,
    values: Vec<ScyllaValue>,
    paging_state: Option<ScyllaPageState>,
) -> NifResult<Atom> {
    let values = values.r()?;
    async_elixir!(env, opaque, {
        let session: &Session = &session.0;
        let res = session.query_paged(query, values, paging_state.r()).await;
        res.ex()
    });
    Ok(atom::ok())
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
        res.ex()
    });
    Ok(atom::ok())
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
        res.ex()
    });
    Ok(atom::ok())
}
