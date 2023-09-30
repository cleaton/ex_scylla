defmodule ExScylla.Native do
  use Rustler,
      otp_app: :ex_scylla,
      crate: "ex_scylla",
      env: if Mix.env() == :test, do: [{"LLVM_PROFILE_FILE", "instrument_coverage.profraw"}, {"RUSTFLAGS", "-C instrument-coverage"}], else: []

      # SessionBuilder
      def sb_default_execution_profile_handle(_sbr, _ephr), do: e()
      #def sb_host_filter(_sbr)
      #def sb_load_balancing(_sbr)
      #def sb_address_translator(_sbr)
      #def sb_authenticator_provider(_sbr)
      #def sb_ssl_context(_sbr)
      def sb_auto_schema_agreement_timeout(_sbr, _timeout_ms), do: e()
      def sb_build(_opaque, _sbr), do: e()
      def sb_compression(_sbr, _compression), do: e()
      def sb_connection_timeout(_sbr, _timeout_ms), do: e()
      def sb_disallow_shard_aware_port(_sbr, _disallow), do: e()
      def sb_fetch_schema_metadata(_sbr, _fetch), do: e()
      def sb_keepalive_interval(_sbr, _interval_ms), do: e()
      def sb_keepalive_timeout(_sbr, _timeout_ms), do: e()
      def sb_keyspaces_to_fetch(_sbr, _keyspaces), do: e()
      def sb_known_node_addr(_sbr, _node_addr), do: e()
      def sb_known_node(_sbr, _hostname), do: e()
      def sb_known_nodes_addr(_sbr, _node_addrs), do: e()
      def sb_known_nodes(_sbr, _hostnames), do: e()
      def sb_new(), do: e()
      def sb_no_auto_schema_agreement(_sbr), do: e()
      def sb_pool_size(_sbr, _size), do: e()
      def sb_refresh_metadata_on_auto_schema_agreement(_sbr, _refresh_metadata), do: e()
      def sb_schema_agreement_interval(_sbr, _interval_ms), do: e()
      def sb_tcp_keepalive_interval(_sbr, _interval_ms), do: e()
      def sb_tcp_nodelay(_sbr, _nodelay), do: e()
      def sb_tracing_info_fetch_attempts(_sbr, _attempts), do: e()
      def sb_tracing_info_fetch_consistency(_sbr, _consistency), do: e()
      def sb_tracing_info_fetch_interval(_sbr, _interval_ms), do: e()
      def sb_user(_sbr, _username, _passwd), do: e()
      def sb_write_coalescing(_sbr, _enablde), do: e()
      # Session
      # //session::s_calculate_token_for_partition_key,
      # //session::s_connect,
      # //session::s_get_default_execution_profile_handle,
      # //session::s_get_keyspace,
      # //session::s_prepare_batch,
      # //session::s_execute_iter,
      # //session::s_get_cluster_data,
      # //session::s_get_metrics,
      # //session::s_get_tracing_info_custom,
      # //session::s_get_tracing_info,
      # //session::s_query_iter,
      def s_await_schema_agreement(_opaque, _session), do: e()
      def s_await_timed_schema_agreement(_opaque, _session, _timeout_ms), do: e()
      def s_batch(_opaque, _session, _batch, _values), do: e()
      def s_calculate_token(_session, _prepared, _values), do: e()
      def s_check_schema_agreement(_opaque, _session), do: e()
      def s_execute_paged(_opaque, _session, _prepared, _values, _paging_state), do: e()
      def s_execute(_opaque, _session, _prepared, _values), do: e()
      def s_fetch_schema_version(_opaque, _session), do: e()
      def s_prepare(_opaque, _session, _query), do: e()
      def s_query_paged(_opaque, _session, _query, _values, _paging_state), do: e()
      def s_query(_opaque_, _session, _query, _values), do: e()
      def s_refresh_metadata(_opaque, _session), do: e()
      def s_use_keyspace(_opaque, _session, _keyspace_name, _case_sensitive), do: e()
      # Query
      # //query::q_get_execution_profile_handle,
      # //query::q_get_request_timeout,
      # //query::q_remove_history_listener,
      # //query::q_set_execution_profile_handle,
      # //query::q_set_history_listener,
      # //query::q_set_request_timeout,
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
      # //batch::b_get_execution_profile_handle,
      # //batch::b_remove_history_listener,
      # //batch::b_set_execution_profile_handle,
      # //batch::b_set_history_listener,
      def b_append_statement(_batch, _statement), do: e()
      def b_get_consistency(_batch), do: e()
      def b_get_is_idempotent(_batch), do: e()
      def b_get_retry_policy(_batch), do: e()
      def b_get_serial_consistency(_batch), do: e()
      def b_get_timestamp(_batch), do: e()
      def b_get_tracing(_batch), do: e()
      def b_get_type(_batch), do: e()
      def b_new_with_statements(_batch_type, _statements), do: e()
      def b_new(_batch_type), do: e()
      def b_set_consistency(_batch, _consistency), do: e()
      def b_set_is_idempotent(_batch, _is_idempotent), do: e()
      def b_set_retry_policy(_batch, _retry_policy), do: n()
      def b_set_serial_consistency(_batch, _sc), do: e()
      def b_set_timestamp(_batch, _timestamp_micros), do: e()
      def b_set_tracing(_batch, _should_trace), do: e()
      # PreparedStatement
      # //prepared_statement::ps_get_execution_profile_handle,
      # //prepared_statement::ps_get_request_timeout,
      # //prepared_statement::ps_is_confirmed_lwt,
      # //prepared_statement::ps_remove_history_listener,
      # //prepared_statement::ps_set_execution_profile_handle,
      # //prepared_statement::ps_set_history_listener,
      # //prepared_statement::set_request_timeout,
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
      def ep_into_handle_with_label(_ep, _label), do: e()
      def ep_into_handle(_ep), do: e()
      def ep_to_builder(_ep), do: e()
      # ExecutionProfileBuilder
      def epb_build(_epb), do: e()
      def epb_consistency(_epb, _consistency), do: e()
      def epb_load_balancing_policy(_epb, _load_balancing_policy), do: e()
      def epb_request_timeout(_epb, _timeout_ms), do: e()
      def epb_retry_policy(_epb, _retry_policy), do: e()
      def epb_serial_consistency(_epb, _serial_consistency), do: e()
      def epb_speculative_execution_policy(_epb, _speculative_execution_policy), do: e()
      # ExecutionProfileHandle
      def eph_map_to_another_profile(_eph, _profile), do: e()
      def eph_pointee_to_builder(_eph), do: e()
      # Defaultpolicy
      def dp_default(), do: e()
      # DefaultPolicyBuilder
      def dpb_build(_dpb), do: e()
      def dpb_enable_shuffling_replicas(_dpb, _enable), do: e()
      def dpb_latency_awareness(_dpb, _latency_awarness_builder), do: e()
      def dpb_new(), do: e()
      def dpb_permit_dc_failover(_dpb, _permit), do: e()
      def dpb_prefer_datacenter(_dpb, _datacenter_name), do: e()
      def dpb_prefer_rack(_dpb, _rack_name), do: e()
      def dpb_token_aware(_dpb, _is_token_aware), do: e()
      # LatencyAwarenessBuilder
      def lab_exclusion_threshold(_lab, _exclusion_threshold), do: e()
      def lab_minimum_measurements(_lab, _minimum_measurements), do: e()
      def lab_new(), do: e()
      def lab_retry_period(_lab, _retry_period_ms), do: e()
      def lab_scale(_lab, _scale_ms), do: e()
      def lab_update_rate(_lab, _update_rate_ms), do: e()

  # helpers
  defp e(), do: :erlang.nif_error(:nif_not_loaded)
  defp n(), do: {:error, :not_implemented_yet}
end
