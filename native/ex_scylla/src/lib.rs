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

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    runtime::init();
    let _ = env.register::<session_builder::types::SessionBuilderResource>();
    let _ = env.register::<session::types::SessionResource>();
    let _ = env.register::<session::types::ScyllaRawRowsResource>();
    let _ = env.register::<batch::types::BatchResource>();
    let _ = env.register::<prepared_statement::types::PreparedStatementResource>();
    let _ = env.register::<query::types::QueryResource>();
    let _ = env.register::<execution::execution_profile_builder::ExecutionProfileBuilderResource>();
    let _ = env.register::<execution::execution_profile_handle::ExecutionProfileHandleResource>();
    let _ = env.register::<execution::execution_profile::ExecutionProfileResource>();
    let _ = env.register::<execution::load_balancing::DefaultPolicyBuilderResource>();
    let _ = env.register::<execution::load_balancing::LatencyAwarenessPolicyBuilderResource>();
    let _ = env.register::<execution::load_balancing::LoadBalancingPolicyResource>();
    true
}
