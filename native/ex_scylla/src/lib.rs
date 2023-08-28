mod batch;
pub mod consts;
pub mod errors;
mod prepared_statement;
mod query;
pub mod runtime;
mod session;
mod session_builder;
pub mod types;
pub mod utils;
use std::option::Option::Some;

// Setup
rustler::init!(
    "Elixir.ExScylla.Native",
    [
        // SessionBuilder
        session_builder::sb_auto_schema_agreement_timeout,
        session_builder::sb_build,
        session_builder::sb_compression,
        session_builder::sb_connection_timeout,
        session_builder::sb_disallow_shard_aware_port,
        session_builder::sb_fetch_schema_metadata,
        session_builder::sb_keepalive_interval,
        session_builder::sb_known_node,
        session_builder::sb_known_node_addr,
        session_builder::sb_known_nodes,
        session_builder::sb_known_nodes_addr,
        session_builder::sb_new,
        session_builder::sb_no_auto_schema_agreement,
        session_builder::sb_pool_size,
        session_builder::sb_schema_agreement_interval,
        session_builder::sb_tcp_nodelay,
        session_builder::sb_use_keyspace,
        session_builder::sb_user,
        // Session
        session::s_await_schema_agreement,
        session::s_await_timed_schema_agreement,
        session::s_batch,
        session::s_check_schema_agreement,
        session::s_execute,
        //session::s_execute_iter,
        session::s_execute_paged,
        session::s_fetch_schema_version,
        //session::s_get_cluster_data,
        //session::s_get_metrics,
        //session::s_get_tracing_info,
        //session::s_get_tracing_info_custom,
        session::s_prepare,
        session::s_query,
        //session::s_query_iter,
        session::s_query_paged,
        session::s_refresh_metadata,
        session::s_use_keyspace,
        //Query
        query::q_disable_paging,
        query::q_get_consistency,
        query::q_get_is_idempotent,
        query::q_get_page_size,
        query::q_get_retry_policy,
        query::q_get_serial_consistency,
        query::q_get_timestamp,
        query::q_get_tracing,
        query::q_new,
        query::q_set_consistency,
        query::q_set_is_idempotent,
        query::q_set_page_size,
        query::q_set_retry_policy,
        query::q_set_serial_consistency,
        query::q_set_timestamp,
        query::q_set_tracing,
        query::q_with_page_size,
        //Batch
        batch::b_append_statement,
        batch::b_get_consistency,
        batch::b_get_is_idempotent,
        batch::b_get_retry_policy,
        batch::b_get_serial_consistency,
        batch::b_get_timestamp,
        batch::b_get_tracing,
        batch::b_get_type,
        batch::b_new,
        batch::b_new_with_statements,
        batch::b_set_consistency,
        batch::b_set_is_idempotent,
        batch::b_set_retry_policy,
        batch::b_set_serial_consistency,
        batch::b_set_timestamp,
        batch::b_set_tracing,
        //PreparedStatement
        prepared_statement::ps_compute_partition_key,
        prepared_statement::ps_disable_paging,
        prepared_statement::ps_get_consistency,
        prepared_statement::ps_get_id,
        prepared_statement::ps_get_is_idempotent,
        prepared_statement::ps_get_keyspace_name,
        prepared_statement::ps_get_page_size,
        prepared_statement::ps_get_prepare_tracing_ids,
        prepared_statement::ps_get_prepared_metadata,
        prepared_statement::ps_get_retry_policy,
        prepared_statement::ps_get_serial_consistency,
        prepared_statement::ps_get_statement,
        prepared_statement::ps_get_table_name,
        prepared_statement::ps_get_timestamp,
        prepared_statement::ps_get_tracing,
        prepared_statement::ps_is_token_aware,
        prepared_statement::ps_set_consistency,
        prepared_statement::ps_set_is_idempotent,
        prepared_statement::ps_set_page_size,
        prepared_statement::ps_set_retry_policy,
        prepared_statement::ps_set_serial_consistency,
        prepared_statement::ps_set_timestamp,
        prepared_statement::ps_set_tracing,
    ],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    runtime::init();
    rustler::resource!(session_builder::types::SessionBuilderResource, env);
    rustler::resource!(session::types::SessionResource, env);
    rustler::resource!(batch::types::BatchResource, env);
    rustler::resource!(prepared_statement::types::PreparedStatementResource, env);
    rustler::resource!(query::types::QueryResource, env);
    true
}
