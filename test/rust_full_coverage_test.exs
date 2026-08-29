defmodule RustFullCoverageTest do
  use ExUnit.Case, async: false

  alias ExScylla.Session
  alias ExScylla.SessionBuilder
  alias ExScylla.Statement.{Batch, Prepared, Query}
  alias ExScylla.Execution.{ExecutionProfile, ExecutionProfileBuilder}
  alias ExScylla.LoadBalancing.{DefaultPolicyBuilder, LatencyAwarenessBuilder}

  alias ExScylla.Types.{
    PercentileSpeculativeExecutionPolicy,
    SimpleSpeculativeExecutionPolicy
  }

  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "rust_full_cov_ks")
    :ok = Session.use_keyspace(session, "rust_full_cov_ks", false)

    {:ok, _} =
      Session.query(
        session,
        """
        CREATE TABLE IF NOT EXISTS test_all_rust_types (
          id INT PRIMARY KEY,
          d DECIMAL,
          v VARINT,
          f FLOAT,
          dbl DOUBLE,
          dt DATE,
          tm TIME,
          ts TIMESTAMP,
          b BOOLEAN,
          bl BLOB,
          txt TEXT,
          asciival ASCII,
          sml SMALLINT,
          tny TINYINT
        );
        """,
        []
      )

    [session: session]
  end

  describe "Statement getters and setters across Query, Prepared, and Batch" do
    test "Query getters and setters", %{session: session} do
      eph = Session.get_default_execution_profile_handle(session)

      q =
        Query.new("SELECT * FROM test_all_rust_types WHERE id = ?;")
        |> Query.set_consistency(:quorum)
        |> Query.set_serial_consistency(:local_serial)
        |> Query.set_is_idempotent(true)
        |> Query.set_page_size(10)
        |> Query.set_request_timeout(5000)
        |> Query.set_timestamp(1_600_000_000)
        |> Query.set_tracing(true)
        |> Query.set_execution_profile_handle(eph)

      assert Query.get_consistency(q) == :quorum
      assert Query.get_serial_consistency(q) == :local_serial
      assert Query.get_is_idempotent(q) == true
      assert Query.get_page_size(q) == 10
      assert Query.get_request_timeout(q) == 5000
      assert Query.get_timestamp(q) == 1_600_000_000
      assert Query.get_tracing(q) == true
      assert is_reference(Query.get_execution_profile_handle(q))
    end

    test "Prepared statement getters and setters", %{session: session} do
      {:ok, ps} = Session.prepare(session, "SELECT * FROM test_all_rust_types WHERE id = ?;")
      eph = Session.get_default_execution_profile_handle(session)

      ps =
        ps
        |> Prepared.set_consistency(:all)
        |> Prepared.set_serial_consistency(:serial)
        |> Prepared.set_is_idempotent(true)
        |> Prepared.set_page_size(25)
        |> Prepared.set_request_timeout(4000)
        |> Prepared.set_timestamp(1_700_000_000)
        |> Prepared.set_tracing(true)
        |> Prepared.set_execution_profile_handle(eph)
        |> Prepared.set_use_cached_result_metadata(true)

      assert Prepared.get_consistency(ps) == :all
      assert Prepared.get_serial_consistency(ps) == :serial
      assert Prepared.get_is_idempotent(ps) == true
      assert Prepared.get_page_size(ps) == 25
      assert Prepared.get_request_timeout(ps) == 4000
      assert Prepared.get_timestamp(ps) == 1_700_000_000
      assert Prepared.get_tracing(ps) == true
      assert Prepared.get_use_cached_result_metadata(ps) == true
      assert is_reference(Prepared.get_execution_profile_handle(ps))
      assert Prepared.is_token_aware(ps) == true
      assert is_binary(Prepared.compute_partition_key(ps, [{:int, 1}]))
    end

    test "Batch getters and setters", %{session: session} do
      eph = Session.get_default_execution_profile_handle(session)

      b =
        Batch.new(:unlogged)
        |> Batch.append_statement("INSERT INTO test_all_rust_types (id) VALUES (?);")
        |> Batch.set_consistency(:two)
        |> Batch.set_serial_consistency(:local_serial)
        |> Batch.set_is_idempotent(true)
        |> Batch.set_request_timeout(6000)
        |> Batch.set_timestamp(1_800_000_000)
        |> Batch.set_tracing(true)
        |> Batch.set_execution_profile_handle(eph)

      assert Batch.get_consistency(b) == :two
      assert Batch.get_serial_consistency(b) == :local_serial
      assert Batch.get_is_idempotent(b) == true
      assert Batch.get_request_timeout(b) == 6000
      assert Batch.get_timestamp(b) == 1_800_000_000
      assert Batch.get_tracing(b) == true
      assert is_reference(Batch.get_execution_profile_handle(b))
    end
  end

  describe "ExecutionProfile and speculative execution policies" do
    test "ExecutionProfileBuilder with speculative execution and retry policies" do
      simple_spec = %SimpleSpeculativeExecutionPolicy{
        max_retry_count: 3,
        retry_interval_ms: 200
      }

      percentile_spec = %PercentileSpeculativeExecutionPolicy{
        max_retry_count: 2,
        percentile: 95.0
      }

      lab =
        LatencyAwarenessBuilder.new()
        |> LatencyAwarenessBuilder.exclusion_threshold(2.0)
        |> LatencyAwarenessBuilder.minimum_measurements(50)
        |> LatencyAwarenessBuilder.retry_period(10_000)
        |> LatencyAwarenessBuilder.scale(100)
        |> LatencyAwarenessBuilder.update_rate(1000)

      dpb =
        DefaultPolicyBuilder.new()
        |> DefaultPolicyBuilder.prefer_datacenter("datacenter1")
        |> DefaultPolicyBuilder.prefer_datacenter_and_rack("datacenter1", "rack1")
        |> DefaultPolicyBuilder.permit_dc_failover(true)
        |> DefaultPolicyBuilder.token_aware(true)
        |> DefaultPolicyBuilder.enable_shuffling_replicas(true)
        |> DefaultPolicyBuilder.latency_awareness(lab)

      lbp = DefaultPolicyBuilder.build(dpb)

      epb =
        ExecutionProfileBuilder.new()
        |> ExecutionProfileBuilder.consistency(:local_quorum)
        |> ExecutionProfileBuilder.serial_consistency(:local_serial)
        |> ExecutionProfileBuilder.request_timeout(8000)
        |> ExecutionProfileBuilder.load_balancing_policy(lbp)
        |> ExecutionProfileBuilder.speculative_execution_policy(simple_spec)
        |> ExecutionProfileBuilder.speculative_execution_policy(percentile_spec)
        |> ExecutionProfileBuilder.retry_policy(:default_retry_policy)
        |> ExecutionProfileBuilder.retry_policy(:fallthrough_retry_policy)

      profile = ExecutionProfileBuilder.build(epb)
      assert is_reference(profile)

      handle = ExecutionProfile.into_handle(profile)
      assert is_reference(handle)
    end
  end

  describe "SessionBuilder configuration options" do
    test "all builder configuration setters" do
      profile = ExecutionProfileBuilder.new() |> ExecutionProfileBuilder.build()
      eph = ExecutionProfile.into_handle(profile)
      node = Application.get_env(:ex_scylla, :test_node, "127.0.0.1:9042")

      sb =
        SessionBuilder.new()
        |> SessionBuilder.known_node(node)
        |> SessionBuilder.connection_timeout(5000)
        |> SessionBuilder.pool_size({:per_host, 2})
        |> SessionBuilder.pool_size({:per_shard, 1})
        |> SessionBuilder.tcp_keepalive_interval(15000)
        |> SessionBuilder.tcp_nodelay(true)
        |> SessionBuilder.tracing_info_fetch_attempts(5)
        |> SessionBuilder.tracing_info_fetch_consistency(:one)
        |> SessionBuilder.tracing_info_fetch_interval(1000)
        |> SessionBuilder.user("test_user", "test_pass")
        |> SessionBuilder.write_coalescing(true)
        |> SessionBuilder.disallow_shard_aware_port(false)
        |> SessionBuilder.auto_schema_agreement_timeout(10000)
        |> SessionBuilder.compression(:lz4)
        |> SessionBuilder.compression(:snappy)
        |> SessionBuilder.prefer_datacenter("datacenter1")
        |> SessionBuilder.prefer_datacenter_and_rack("datacenter1", "rack1")
        |> SessionBuilder.prefer_no_datacenter()
        |> SessionBuilder.default_execution_profile_handle(eph)

      assert is_reference(sb)
    end
  end

  describe "Rust type serialization and CqlValue mapping" do
    test "CqlValue conversions for Decimal, Varint, and standard types", %{session: session} do
      {:ok, _} =
        Session.query(
          session,
          "INSERT INTO test_all_rust_types (id, d, v, b, bl, txt, asciival, sml, tny) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);",
          [
            {:int, 7001},
            {:decimal, "-123.45600"},
            {:varint, "-999999999999999999"},
            {:boolean, false},
            {:blob, <<0xFF, 0x00>>},
            {:text, "unicode_test"},
            {:ascii, "ascii_clean"},
            {:small_int, -1000},
            {:tiny_int, -50}
          ]
        )

      {:ok, res} =
        Session.query(
          session,
          "SELECT id, d, v, b, bl, txt, asciival, sml, tny FROM test_all_rust_types WHERE id = 7001;",
          []
        )

      assert length(res.rows) == 1
    end
  end
end
