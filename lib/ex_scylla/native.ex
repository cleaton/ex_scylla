defmodule ExScylla.Native do
  use Rustler,
    otp_app: :ex_scylla,
    crate: "ex_scylla",
    env: if(Mix.env() == :test, do: [{"RUSTFLAGS", "-C instrument-coverage"}], else: [])

  # SessionBuilder
  def sb_auto_schema_agreement_timeout(_sbr, _timeout_ms), do: e()
  def sb_build(_opaque, _sbr), do: e()
  def sb_compression(_sbr, _compression), do: e()
  def sb_connection_timeout(_sbr, _timeout_ms), do: e()
  def sb_default_consistency(_sbr, _consistency), do: e()
  def sb_disallow_shard_aware_port(_sbr, _disallow), do: e()
  def sb_fetch_schema_metadata(_sbr, _fetch), do: e()
  def sb_keepalive_interval(_sbr, _interval_ms), do: e()
  def sb_known_node(_sbr, _hostname), do: e()
  def sb_known_node_addr(_sbr, _node_addr), do: e()
  def sb_known_nodes(_sbr, _hostnames), do: e()
  def sb_known_nodes_addr(_sbr, _node_addrs), do: e()
  def sb_load_balancing(_sbr, _policy), do: e()
  def sb_new(), do: e()
  def sb_no_auto_schema_agreement(_sbr), do: e()
  def sb_pool_size(_sbr, _size), do: e()
  def sb_retry_policy(_sbr, _retry_policy), do: e()
  def sb_schema_agreement_interval(_sbr, _interval_ms), do: e()
  def sb_speculative_execution(_sbr, _policy), do: e()
  def sb_tcp_nodelay(_sbr, _nodelay), do: e()
  def sb_use_keyspace(_sbr, _keyspace_name, _case_sensitive), do: e()
  def sb_user(_sbr, _username, _passwd), do: e()
  def sb_default_execution_profile_handle(_sbr, _handle), do: e()
  def sb_keepalive_timeout(_sbr, _timeout_ms), do: e()

  # Session
  def s_await_schema_agreement(_opaque, _session), do: e()
  def s_await_timed_schema_agreement(_opaque, _session, _timeout_ms), do: e()
  def s_batch(_opaque, _session, _batch, _values), do: e()
  def s_check_schema_agreement(_opaque, _session), do: e()
  def s_execute(_opaque, _session, _prepared, _values), do: e()
  # //session::s_execute_iter,
  def s_execute_paged(_opaque, _session, _prepared, _values, _paging_state), do: e()
  def s_fetch_schema_version(_opaque, _session), do: e()
  # //session::s_get_cluster_data,
  # //session::s_get_metrics,
  # //session::s_get_tracing_info,
  # //session::s_get_tracing_info_custom,
  def s_prepare(_opaque, _session, _query), do: e()
  def s_query(_opaque_, _session, _query, _values), do: e()
  # //session::s_query_iter,
  def s_query_paged(_opaque, _session, _query, _values, _paging_state), do: e()
  def s_refresh_metadata(_opaque, _session), do: e()
  def s_use_keyspace(_opaque, _session, _keyspace_name, _case_sensitive), do: e()

  # Query
  def q_disable_paging(_q), do: e()
  def q_get_consistency(_q), do: e()
  def q_get_is_idempotent(_q), do: e()
  def q_get_page_size(_q), do: e()
  def q_get_retry_policy(_q), do: e()
  def q_get_serial_consistency(_q), do: e()
  def q_get_timestamp(_q), do: e()
  def q_get_tracing(_q), do: e()
  def q_new(_query_text), do: e()
  def q_set_consistency(_q, _consistency), do: e()
  def q_set_is_idempotent(_q, _is_idempotent), do: e()
  def q_set_page_size(_q, _page_size), do: e()
  def q_set_retry_policy(_q), do: n()
  def q_set_serial_consistency(_q, _sc), do: e()
  def q_set_timestamp(_q, _timestamp), do: e()
  def q_set_tracing(_q, _should_trace), do: e()
  def q_with_page_size(_q, _page_size), do: e()

  # Batch
  def b_append_statement(_batch, _statement), do: e()
  def b_get_consistency(_batch), do: e()
  def b_get_is_idempotent(_batch), do: e()
  def b_get_retry_policy(_batch), do: e()
  def b_get_serial_consistency(_batch), do: e()
  def b_get_timestamp(_batch), do: e()
  def b_get_tracing(_batch), do: e()
  def b_get_type(_batch), do: e()
  def b_new(_batch_type), do: e()
  def b_new_with_statements(_batch_type, _statements), do: e()
  def b_set_consistency(_batch, _consistency), do: e()
  def b_set_is_idempotent(_batch, _is_idempotent), do: e()
  def b_set_retry_policy(_batch, _retry_policy), do: n()
  def b_set_serial_consistency(_batch, _sc), do: e()
  def b_set_timestamp(_batch, _timestamp_micros), do: e()
  def b_set_tracing(_batch, _should_trace), do: e()

  # PreparedStatement
  def ps_compute_partition_key(_ps, _bound_values), do: e()
  def ps_disable_paging(_ps), do: e()
  def ps_get_consistency(_ps), do: e()
  def ps_get_id(_ps), do: e()
  def ps_get_is_idempotent(_ps), do: e()
  def ps_get_keyspace_name(_ps), do: e()
  def ps_get_page_size(_ps), do: e()
  def ps_get_prepare_tracing_ids(_ps), do: e()
  def ps_get_prepared_metadata(_ps), do: e()
  def ps_get_retry_policy(_ps), do: e()
  def ps_get_serial_consistency(_ps), do: e()
  def ps_get_statement(_ps), do: e()
  def ps_get_table_name(_ps), do: e()
  def ps_get_timestamp(_ps), do: e()
  def ps_get_tracing(_ps), do: e()
  def ps_is_token_aware(_ps), do: e()
  def ps_set_consistency(_ps, _consistency), do: e()
  def ps_set_is_idempotent(_ps, _is_idempotent), do: e()
  def ps_set_page_size(_ps, _page_size), do: e()
  def ps_set_retry_policy(_ps), do: n()
  def ps_set_serial_consistency(_ps, _sc), do: e()
  def ps_set_timestamp(_ps, _timestamp_micros), do: e()
  def ps_set_tracing(_ps, _should_trace), do: e()

  # ExecutionProfile
  def ep_builder(), do: e()
  def ep_request_timeout(_ep, _timeout_ms), do: e()
  def ep_consistency(_ep, _consistency), do: e()
  def ep_serial_consistency(_ep, _serial_consistency), do: e()
  def ep_load_balancing_policy(_ep, _lb_policy), do: e()
  def ep_retry_policy(_ep, _retry_policy), do: e()
  def ep_speculative_execution_policy(_ep, _speculative_execution_policy), do: e()
  def ep_build(_ep), do: e()
  def ep_into_handle(_ep), do: e()

  # helpers
  defp e(), do: :erlang.nif_error(:nif_not_loaded)
  defp n(), do: {:error, :not_implemented_yet}
end
