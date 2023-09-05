pub mod types;
use rustler::env::{OwnedEnv, SavedTerm};
//use consistency::Consistency;
use super::runtime;
use crate::settings::pool_size::ScyllaPoolSize;
use rustler::types::atom;
use rustler::{Atom, Encoder, Env, NifResult, ResourceArc, Term};
use scylla::SessionBuilder;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Mutex;
use std::time::Duration;
//use rustler::types::LocalPid;
use crate::types::*;
use crate::utils::*;
use std::cell::Cell;
use types::*;

macro_rules! use_builder {
    ($sbr:ident, $e:expr) => {
        let guard = $sbr.lock().unwrap();
        guard.set($e(guard.take()));
        drop(guard);
    };
}

// SesisonBuilder methods
#[rustler::nif]
fn sb_write_coalescing(
    sbr: ResourceArc<SessionBuilderResource>,
    enable: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.write_coalescing(enable) });
    sbr
}

#[rustler::nif]
fn sb_auto_schema_agreement_timeout(
    sbr: ResourceArc<SessionBuilderResource>,
    timeout_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.auto_schema_agreement_timeout(Duration::from_millis(timeout_ms))
    });
    sbr
}

#[rustler::nif]
fn sb_build<'a>(
    env: Env<'a>,
    opaque: Term<'a>,
    sbr: ResourceArc<SessionBuilderResource>,
) -> NifResult<Atom> {
    let sb: SessionBuilder = sbr.lock().unwrap().take();
    async_elixir!(env, opaque, {
        let res = sb.build().await;
        res.ex()
    });
    Ok(atom::ok())
}

#[rustler::nif]
fn sb_compression(
    sbr: ResourceArc<SessionBuilderResource>,
    compression: ScyllaTransportCompression,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.compression(Some(compression.into()))
    });
    sbr
}

#[rustler::nif]
fn sb_connection_timeout(
    sbr: ResourceArc<SessionBuilderResource>,
    timeout_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.connection_timeout(Duration::from_millis(timeout_ms))
    });
    sbr
}

/*
#[rustler::nif]
fn sb_default_execution_profile_handle(
    sbr: ResourceArc<SessionBuilderResource>,
    profile_handle: ExecutionProfileHandle,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.default_execution_profile_handle(profile_handle)
    });
    sbr
}
*/

#[rustler::nif]
fn sb_disallow_shard_aware_port(
    sbr: ResourceArc<SessionBuilderResource>,
    disallow: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.disallow_shard_aware_port(disallow)
    });
    sbr
}
#[rustler::nif]
fn sb_fetch_schema_metadata(
    sbr: ResourceArc<SessionBuilderResource>,
    fetch: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.fetch_schema_metadata(fetch)
    });
    sbr
}

/*
#[rustler::nif]
fn sb_host_filter(
    sbr: ResourceArc<SessionBuilderResource>,
    filter: Arc<dyn HostFilter>,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.host_filter(filter) });
    sbr
}
 */

#[rustler::nif]
fn sb_keepalive_interval(
    sbr: ResourceArc<SessionBuilderResource>,
    interval_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.keepalive_interval(Duration::from_millis(interval_ms))
    });
    sbr
}

#[rustler::nif]
fn sb_keepalive_timeout(
    sbr: ResourceArc<SessionBuilderResource>,
    timeout_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.keepalive_timeout(Duration::from_millis(timeout_ms))
    });
    sbr
}

#[rustler::nif]
fn sb_keyspaces_to_fetch(
    sbr: ResourceArc<SessionBuilderResource>,
    keyspaces: Vec<String>,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.keyspaces_to_fetch(keyspaces)
    });
    sbr
}

#[rustler::nif]
fn sb_known_node(
    sbr: ResourceArc<SessionBuilderResource>,
    hostname: String,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.known_node(hostname) });
    sbr
}

#[rustler::nif]
fn sb_known_node_addr(
    sbr: ResourceArc<SessionBuilderResource>,
    node_addr: ScyllaSocketAddr,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.known_node_addr(node_addr.into())
    });
    sbr
}

#[rustler::nif]
fn sb_known_nodes(
    sbr: ResourceArc<SessionBuilderResource>,
    hostnames: Vec<String>,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.known_nodes(hostnames.as_slice())
    });
    sbr
}

#[rustler::nif]
fn sb_known_nodes_addr(
    sbr: ResourceArc<SessionBuilderResource>,
    node_addrs: Vec<ScyllaSocketAddr>,
) -> ResourceArc<SessionBuilderResource> {
    let node_addrs: Vec<SocketAddr> = node_addrs.into_iter().map(|a| a.into()).collect();
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.known_nodes_addr(node_addrs.as_slice())
    });
    sbr
}

/*
#[rustler::nif]
fn sb_load_balancing(
    sbr: ResourceArc<SessionBuilderResource>,
    policy: ScyllaLoadBalancingPolicy,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.load_balancing(policy.r()) });
    sbr
}
*/

#[rustler::nif]
fn sb_new() -> ResourceArc<SessionBuilderResource> {
    ResourceArc::new(SessionBuilderResource(Mutex::new(Cell::new(
        SessionBuilder::new(),
    ))))
}
#[rustler::nif]
fn sb_no_auto_schema_agreement(
    sbr: ResourceArc<SessionBuilderResource>,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.no_auto_schema_agreement() });
    sbr
}
#[rustler::nif]
fn sb_pool_size(
    sbr: ResourceArc<SessionBuilderResource>,
    size: ScyllaPoolSize,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.pool_size(size.into()) });
    sbr
}

#[rustler::nif]
fn sb_refresh_metadata_on_auto_schema_agreement(
    sbr: ResourceArc<SessionBuilderResource>,
    refresh_metadata: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.refresh_metadata_on_auto_schema_agreement(refresh_metadata)
    });
    sbr
}

#[rustler::nif]
fn sb_schema_agreement_interval(
    sbr: ResourceArc<SessionBuilderResource>,
    interval_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.schema_agreement_interval(Duration::from_millis(interval_ms))
    });
    sbr
}

#[rustler::nif]
fn sb_tcp_keepalive_interval(
    sbr: ResourceArc<SessionBuilderResource>,
    interval_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.tcp_keepalive_interval(Duration::from_millis(interval_ms))
    });
    sbr
}

#[rustler::nif]
fn sb_tcp_nodelay(
    sbr: ResourceArc<SessionBuilderResource>,
    nodelay: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.tcp_nodelay(nodelay) });
    sbr
}

#[rustler::nif]
fn sb_tracing_info_fetch_attempts(
    sbr: ResourceArc<SessionBuilderResource>,
    attempts: u32,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.tracing_info_fetch_attempts(NonZeroU32::new(attempts).expect("attemts can not be 0"))
    });
    sbr
}

#[rustler::nif]
fn sb_tracing_info_fetch_consistency(
    sbr: ResourceArc<SessionBuilderResource>,
    consistency: ScyllaConsistency,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.tracing_info_fetch_consistency(consistency.into())
    });
    sbr
}

#[rustler::nif]
fn sb_tracing_info_fetch_interval(
    sbr: ResourceArc<SessionBuilderResource>,
    interval_ms: u64,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.tracing_info_fetch_interval(Duration::from_millis(interval_ms))
    });
    sbr
}
#[rustler::nif]
fn sb_use_keyspace(
    sbr: ResourceArc<SessionBuilderResource>,
    keyspace_name: String,
    case_sensitive: bool,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| {
        sb.use_keyspace(keyspace_name, case_sensitive)
    });
    sbr
}

#[rustler::nif]
fn sb_user(
    sbr: ResourceArc<SessionBuilderResource>,
    username: String,
    passwd: String,
) -> ResourceArc<SessionBuilderResource> {
    use_builder!(sbr, |sb: SessionBuilder| { sb.user(username, passwd) });
    sbr
}
