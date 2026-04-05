mod batch;
pub mod consts;
pub mod errors;
mod execution;
mod prepared_statement;
mod query;
pub mod runtime;
mod session;
mod session_builder;
pub mod types;
pub mod utils;

// Setup
rustler::init!("Elixir.ExScylla.Native", load = load);

#[allow(non_local_definitions)]
fn load(env: rustler::Env, _: rustler::Term) -> bool {
    runtime::init();
    let _ = rustler::resource!(session_builder::types::SessionBuilderResource, env);
    let _ = rustler::resource!(session::types::SessionResource, env);
    let _ = rustler::resource!(session::types::ScyllaRawRowsResource, env);
    let _ = rustler::resource!(batch::types::BatchResource, env);
    let _ = rustler::resource!(prepared_statement::types::PreparedStatementResource, env);
    let _ = rustler::resource!(query::types::QueryResource, env);
    let _ = rustler::resource!(
        execution::execution_profile_builder::ExecutionProfileBuilderResource,
        env
    );
    let _ = rustler::resource!(
        execution::execution_profile_handle::ExecutionProfileHandleResource,
        env
    );
    let _ = rustler::resource!(execution::execution_profile::ExecutionProfileResource, env);
    let _ = rustler::resource!(execution::load_balancing::DefaultPolicyBuilderResource, env);
    let _ = rustler::resource!(
        execution::load_balancing::LatencyAwarenessPolicyBuilderResource,
        env
    );
    let _ = rustler::resource!(execution::load_balancing::LoadBalancingPolicyResource, env);
    true
}
