#![allow(non_local_definitions)]

mod batch;
pub mod consts;
pub mod errors;
mod prepared_statement;
mod query;
pub mod runtime;
mod session;
mod session_builder;
mod execution;
pub mod types;
pub mod utils;
use std::option::Option::Some;
use rustler::Env;



/// Declares all NIF resources used by the library.
/// Returns true if all resources were successfully declared, false otherwise.
fn declare_resources(env: Env) -> bool {
    let sb_res = rustler::resource!(session_builder::types::SessionBuilderResource, env);
    let s_res = rustler::resource!(session::types::SessionResource, env);
    let b_res = rustler::resource!(batch::types::BatchResource, env);
    let ps_res = rustler::resource!(prepared_statement::types::PreparedStatementResource, env);
    let q_res = rustler::resource!(query::types::QueryResource, env);
    let epb_res = rustler::resource!(
        execution::execution_profile_builder::ExecutionProfileBuilderResource,
        env
    );
    let eph_res = rustler::resource!(
        execution::execution_profile_handle::ExecutionProfileHandleResource,
        env
    );
    let ep_res = rustler::resource!(execution::execution_profile::ExecutionProfileResource, env);
    let dpb_res = rustler::resource!(execution::load_balancing::DefaultPolicyBuilderResource, env);
    let lab_res = rustler::resource!(
        execution::load_balancing::LatencyAwarenessPolicyBuilderResource,
        env
    );
    let lbp_res = rustler::resource!(execution::load_balancing::LoadBalancingPolicyResource, env);
    
    sb_res && s_res && b_res && ps_res && q_res && epb_res && eph_res && ep_res && dpb_res && lab_res && lbp_res
}

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    println!("Loading ExScylla.Native...");
    runtime::init();
    declare_resources(env);
    true
}

// Setup
rustler::init!("Elixir.ExScylla.Native", load = load);