defmodule ExScylla.Examples.CqlTypesTest do
  use ExUnit.Case, async: false
  alias ExScylla.Session
  alias ExScylla.TestSupport

  setup_all do
    session = TestSupport.get_session()
    TestSupport.setup_simple_keyspace(session, "cql_types_test")
    :ok = Session.use_keyspace(session, "cql_types_test", false)

    create_table = """
    CREATE TABLE IF NOT EXISTS all_cql_types (
      id INT PRIMARY KEY,
      txt TEXT,
      big BIGINT,
      blb BLOB,
      bll BOOLEAN,
      dt DATE,
      tm TIME,
      ts TIMESTAMP,
      dbl DOUBLE,
      flt FLOAT,
      sml SMALLINT,
      tny TINYINT,
      uid UUID,
      lst LIST<TEXT>,
      st SET<INT>,
      mp MAP<TEXT, INT>
    );
    """

    {:ok, _} = Session.query(session, create_table, [])
    [session: session]
  end

  test "insert and retrieve all standard and collection CQL types", %{session: session} do
    uuid_raw =
      <<0x12, 0x3E, 0x45, 0x67, 0xE8, 0x9B, 0x12, 0xD3, 0xA4, 0x56, 0x42, 0x66, 0x14, 0x17, 0x40,
        0x00>>

    insert = """
    INSERT INTO all_cql_types (
      id, txt, big, blb, bll, dt, tm, ts, dbl, flt, sml, tny, uid, lst, st, mp
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """

    values = [
      {:int, 1},
      {:text, "hello"},
      {:big_int, 9_223_372_036_854_775_807},
      {:blob, <<1, 2, 3, 4>>},
      {:boolean, true},
      {:date, 18250},
      {:time, 3_600_000_000_000},
      {:timestamp, 1_693_000_000_000},
      {:double, 3.14159},
      {:float, 2.5},
      {:small_int, 32767},
      {:tiny_int, 127},
      {:uuid, uuid_raw},
      {:list, [{:text, "a"}, {:text, "b"}]},
      {:set, [{:int, 10}, {:int, 20}]},
      {:map, [{{:text, "k1"}, {:int, 100}}]}
    ]

    {:ok, _} = Session.query(session, insert, values)

    select = "SELECT * FROM all_cql_types WHERE id = 1"
    {:ok, res} = Session.query(session, select, [])
    assert length(res.rows) == 1
  end
end
