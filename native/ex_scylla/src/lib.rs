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
use execution::execution_profile;
use execution::execution_profile_builder;
use execution::execution_profile_handle;
use execution::load_balancing;

// Setup
rustler::init!(
    "Elixir.ExScylla.Native",
    [
        // SessionBuilder
        // GenericSessionBuilder
        session_builder::sb_default_execution_profile_handle,
        //session_builder::sb_host_filter,
        //session_builder::sb_address_translator,
        //session_builder::sb_authenticator_provider,
        //session_builder::sb_ssl_context,
        session_builder::sb_auto_schema_agreement_timeout,
        session_builder::sb_build,
        session_builder::sb_compression,
        session_builder::sb_connection_timeout,
        session_builder::sb_disallow_shard_aware_port,
        session_builder::sb_fetch_schema_metadata,
        session_builder::sb_keepalive_interval,
        session_builder::sb_keepalive_timeout,
        session_builder::sb_keyspaces_to_fetch,
        session_builder::sb_known_node_addr,
        session_builder::sb_known_node,
        session_builder::sb_known_nodes_addr,
        session_builder::sb_known_nodes,
        session_builder::sb_new,
        session_builder::sb_no_auto_schema_agreement,
        session_builder::sb_pool_size,
        session_builder::sb_refresh_metadata_on_auto_schema_agreement,
        session_builder::sb_schema_agreement_interval,
        session_builder::sb_tcp_keepalive_interval,
        session_builder::sb_tcp_nodelay,
        session_builder::sb_tracing_info_fetch_attempts,
        session_builder::sb_tracing_info_fetch_consistency,
        session_builder::sb_tracing_info_fetch_interval,
        session_builder::sb_user,
        session_builder::sb_write_coalescing,
        // SessionConfig
        /*
        address_translator
        authenticator
        auto_await_schema_agreement_timeout
        cloud_config
        compression
        connect_timeout
        connection_pool_size
        default_execution_profile_handle
        disallow_shard_aware_port
        enable_write_coalescing
        fetch_schema_metadata
        host_filter
        keepalive_interval
        keepalive_timeout
        keyspace_case_sensitive
        keyspaces_to_fetch
        known_nodes
        refresh_metadata_on_auto_schema_agreement
        schema_agreement_interval
        ssl_context
        tcp_keepalive_interval
        tcp_nodelay
        tracing_info_fetch_attempts
        tracing_info_fetch_consistency
        tracing_info_fetch_interval
        used_keyspace
        */
        // Session
        //session::s_calculate_token_for_partition_key,
        //session::s_connect,
        //session::s_get_default_execution_profile_handle,
        //session::s_get_keyspace,
        //session::s_prepare_batch,
        //session::s_execute_iter,
        //session::s_get_cluster_data,
        //session::s_get_metrics,
        //session::s_get_tracing_info,
        //session::s_query_iter,
        session::s_await_schema_agreement,
        session::s_await_timed_schema_agreement,
        session::s_batch,
        session::s_calculate_token,
        session::s_check_schema_agreement,
        session::s_execute_paged,
        session::s_execute,
        session::s_fetch_schema_version,
        session::s_prepare,
        session::s_query_paged,
        session::s_query,
        session::s_refresh_metadata,
        session::s_use_keyspace,
        //Query
        query::q_get_execution_profile_handle,
        query::q_get_request_timeout,
        //query::q_remove_history_listener,
        query::q_set_execution_profile_handle,
        //query::q_set_history_listener,
        query::q_set_request_timeout,
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
        //batch::b_get_execution_profile_handle,
        //batch::b_remove_history_listener,
        //batch::b_set_execution_profile_handle,
        //batch::b_set_history_listener,
        batch::b_append_statement,
        batch::b_get_consistency,
        batch::b_get_is_idempotent,
        batch::b_get_retry_policy,
        batch::b_get_serial_consistency,
        batch::b_get_timestamp,
        batch::b_get_tracing,
        batch::b_get_type,
        batch::b_new_with_statements,
        batch::b_new,
        batch::b_set_consistency,
        batch::b_set_is_idempotent,
        batch::b_set_retry_policy,
        batch::b_set_serial_consistency,
        batch::b_set_timestamp,
        batch::b_set_tracing,
        //PreparedStatement
        //prepared_statement::ps_get_execution_profile_handle,
        //prepared_statement::ps_get_request_timeout,
        //prepared_statement::ps_is_confirmed_lwt,
        //prepared_statement::ps_remove_history_listener,
        //prepared_statement::ps_set_execution_profile_handle,
        //prepared_statement::ps_set_history_listener,
        //prepared_statement::set_request_timeout,
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
        // ExecutionProfile
        execution_profile::ep_builder,
        execution_profile::ep_into_handle_with_label,
        execution_profile::ep_into_handle,
        execution_profile::ep_to_builder,
        // ExecutionProfileBuilder
        execution_profile_builder::epb_build,
        execution_profile_builder::epb_consistency,
        execution_profile_builder::epb_load_balancing_policy,
        execution_profile_builder::epb_request_timeout,
        execution_profile_builder::epb_retry_policy,
        execution_profile_builder::epb_serial_consistency,
        execution_profile_builder::epb_speculative_execution_policy,
        // ExecutionProfileHandle
        execution_profile_handle::eph_map_to_another_profile,
        execution_profile_handle::eph_pointee_to_builder,
        // DefaultPolicy
        load_balancing::dp_default,
        // DefaultPolicyBuilder
        load_balancing::dpb_build,
        load_balancing::dpb_enable_shuffling_replicas,
        load_balancing::dpb_latency_awareness,
        load_balancing::dpb_new,
        load_balancing::dpb_permit_dc_failover,
        load_balancing::dpb_prefer_datacenter,
        load_balancing::dpb_prefer_rack,
        load_balancing::dpb_token_aware,
        // LatencyAwarenessBuilder
        load_balancing::lab_exclusion_threshold,
        load_balancing::lab_minimum_measurements,
        load_balancing::lab_new,
        load_balancing::lab_retry_period,
        load_balancing::lab_scale,
        load_balancing::lab_update_rate,
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
    rustler::resource!(
        execution::execution_profile_builder::ExecutionProfileBuilderResource,
        env
    );
    rustler::resource!(
        execution::execution_profile_handle::ExecutionProfileHandleResource,
        env
    );
    rustler::resource!(execution::execution_profile::ExecutionProfileResource, env);
    rustler::resource!(execution::load_balancing::DefaultPolicyBuilderResource, env);
    rustler::resource!(
        execution::load_balancing::LatencyAwarenessPolicyBuilderResource,
        env
    );
    rustler::resource!(execution::load_balancing::LoadBalancingPolicyResource, env);
    true
}
