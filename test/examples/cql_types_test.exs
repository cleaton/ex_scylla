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

  describe "types fixed in the 2026-11 upgrade" do
    setup %{session: session} do
      for stmt <- [
            "CREATE TYPE IF NOT EXISTS address (street TEXT, zip INT)",
            """
            CREATE TABLE IF NOT EXISTS extra_types (
              id INT PRIMARY KEY,
              addr FROZEN<address>,
              dur DURATION,
              vi VARINT,
              dec DECIMAL
            )
            """
          ] do
        {:ok, _} = Session.query(session, stmt, [])
      end

      :ok
    end

    test "UDT columns decode to a map keyed by field name", %{session: session} do
      {:ok, _} =
        Session.query(
          session,
          "INSERT INTO extra_types (id, addr) VALUES (1, {street: 'Main', zip: 12345})",
          []
        )

      {:ok, res} = Session.query(session, "SELECT addr FROM extra_types WHERE id = 1", [])

      assert [{:user_defined_type, %ExScylla.Types.UserDefinedColumnType{} = udt}] =
               res.column_types

      assert [{"street", :text}, {"zip", :int}] = udt.field_types

      assert [%ExScylla.Types.Row{columns: [{_type, %{"street" => "Main", "zip" => 12345}}]}] =
               res.rows
    end

    test "duration columns decode to CqlDuration", %{session: session} do
      duration = %ExScylla.Types.CqlDuration{months: 14, days: 3, nanoseconds: 3_600_000_000_000}

      {:ok, _} =
        Session.query(session, "INSERT INTO extra_types (id, dur) VALUES (?, ?)", [
          {:int, 2},
          {:duration, duration}
        ])

      {:ok, res} = Session.query(session, "SELECT dur FROM extra_types WHERE id = 2", [])
      assert [%ExScylla.Types.Row{columns: [{:duration, ^duration}]}] = res.rows
    end

    test "varint and decimal round-trip through num-bigint 0.5 and bigdecimal", %{
      session: session
    } do
      big = "-123456789012345678901234567890"
      dec = "-98765432109876543210.0012300"

      {:ok, _} =
        Session.query(session, "INSERT INTO extra_types (id, vi, dec) VALUES (?, ?, ?)", [
          {:int, 4},
          {:varint, big},
          {:decimal, dec}
        ])

      {:ok, res} = Session.query(session, "SELECT vi, dec FROM extra_types WHERE id = 4", [])
      assert [%ExScylla.Types.Row{columns: [{:varint, ^big}, {:decimal, ^dec}]}] = res.rows
    end

    test "nil binds NULL, and empty values decode without crashing", %{session: session} do
      {:ok, _} =
        Session.query(
          session,
          "CREATE TABLE IF NOT EXISTS null_types (id INT PRIMARY KEY, v INT, txt TEXT, u UUID)",
          []
        )

      insert = "INSERT INTO null_types (id, v, txt, u) VALUES (?, ?, ?, ?)"
      {:ok, ps} = Session.prepare(session, insert)

      {:ok, _} = Session.query(session, insert, [{:int, 1}, nil, nil, nil])
      {:ok, _} = Session.execute(session, ps, [{:int, 2}, nil, {:text, "x"}, nil])
      {:ok, _} = Session.query(session, insert, [{:int, 3}, :empty, :empty, :empty])

      select = "SELECT v, txt, u FROM null_types WHERE id = ?"

      for {id, expected} <- [{1, [nil, nil, nil]}, {2, [nil, "x", nil]}, {3, [nil, "", nil]}] do
        {:ok, res} = Session.query(session, select, [{:int, id}])
        assert [%ExScylla.Types.Row{columns: cols}] = res.rows

        assert Enum.map(cols, fn
                 nil -> nil
                 {_type, v} -> v
               end) == expected

        {:ok, raw} = Session.query_raw(session, select, [{:int, id}])
        assert raw.rows == [expected]
      end
    end

    test "uuid and timeuuid accept hyphenated strings as well as 16-byte binaries", %{
      session: session
    } do
      {:ok, _} =
        Session.query(
          session,
          "CREATE TABLE IF NOT EXISTS uuid_types (id INT PRIMARY KEY, u UUID, t TIMEUUID)",
          []
        )

      uuid = "123e4567-e89b-12d3-a456-426614174000"
      timeuuid = "50554d6e-29bb-11e5-b345-feff819cdc9f"

      {:ok, _} =
        Session.query(session, "INSERT INTO uuid_types (id, u, t) VALUES (?, ?, ?)", [
          {:int, 1},
          {:uuid, uuid},
          {:timeuuid, String.upcase(timeuuid)}
        ])

      {:ok, _} =
        Session.query(session, "INSERT INTO uuid_types (id, u, t) VALUES (?, ?, ?)", [
          {:int, 2},
          {:uuid, Base.decode16!(String.replace(uuid, "-", ""), case: :lower)},
          {:timeuuid, Base.decode16!(String.replace(timeuuid, "-", ""), case: :lower)}
        ])

      {:ok, res} = Session.query(session, "SELECT u, t FROM uuid_types WHERE id IN (1, 2)", [])

      assert [row, row] = Enum.map(res.rows, & &1.columns)
      assert [{:uuid, ^uuid}, {:timeuuid, ^timeuuid}] = row
    end
  end
end
