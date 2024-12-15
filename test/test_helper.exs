alias ExScylla.TestSupport

{_container, node, session} = TestSupport.start_container()

# Set the test node configuration
Application.put_env(:ex_scylla, :test_node, node)

# Create ETS table for test resources
:ets.new(:ex_scylla_test, [:set, :public, :named_table])
:ets.insert(:ex_scylla_test, {:session, session})

# Setup keyspaces needed for tests
TestSupport.setup_simple_keyspace(session, "test")
TestSupport.setup_simple_keyspace(session, "another_test_keyspace")

ExUnit.start()
