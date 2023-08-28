alias ExScylla.Session
alias ExScylla.SessionBuilder

{:ok, session} =
  SessionBuilder.new()
  |> SessionBuilder.known_node("127.0.0.1:9042")
  |> SessionBuilder.build()

ks1 = """
  CREATE KEYSPACE IF NOT EXISTS test
  WITH REPLICATION = {
    'class' : 'SimpleStrategy',
    'replication_factor' : 1
  };
"""

ks2 = """
  CREATE KEYSPACE IF NOT EXISTS another_test_keyspace
  WITH REPLICATION = {
    'class' : 'SimpleStrategy',
    'replication_factor' : 1
  };
"""

{:ok, _} = Session.query(session, ks1, [])
{:ok, _} = Session.query(session, ks2, [])
ExUnit.start()
