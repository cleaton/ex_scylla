# Type System & CQL Mapping

ExScylla provides bidirectional mapping between Apache Cassandra / ScyllaDB CQL types, Rust driver types (`scylla::value::CqlValue`), and Elixir terms.

## Data Type Mapping Reference

| CQL Type | Rust Driver (`CqlValue`) | Elixir Term (Decoded API) | Elixir Input Tuple |
| :--- | :--- | :--- | :--- |
| `ascii` / `varchar` / `text` | `CqlValue::Text(String)` | `{:text, binary()}` | `{:text, "string"}` |
| `bigint` / `counter` | `CqlValue::BigInt(i64)` | `{:bigint, integer()}` | `{:bigint, 123456789}` |
| `blob` | `CqlValue::Blob(Vec<u8>)` | `{:blob, binary()}` | `{:blob, <<1, 2, 3>>}` |
| `boolean` | `CqlValue::Boolean(bool)` | `{:boolean, boolean()}` | `{:boolean, true}` |
| `date` | `CqlValue::Date(u32)` | `{:date, integer()}` | `{:date, 18250}` |
| `decimal` | `CqlValue::Decimal(CqlDecimal)` | `{:decimal, binary()}` | `{:decimal, "123.45"}` |
| `double` | `CqlValue::Double(f64)` | `{:double, float()}` | `{:double, 3.14159}` |
| `duration` | `CqlValue::Duration(CqlDuration)` | `{:duration, %CqlDuration{}}` | `{:duration, %CqlDuration{...}}` |
| `float` | `CqlValue::Float(f32)` | `{:float, float()}` | `{:float, 2.718}` |
| `inet` | `CqlValue::Inet(IpAddr)` | `{:inet, tuple() \| binary()}` | `{:inet, {127, 0, 0, 1}}` |
| `int` | `CqlValue::Int(i32)` | `{:int, integer()}` | `{:int, 42}` |
| `smallint` | `CqlValue::SmallInt(i16)` | `{:smallint, integer()}` | `{:smallint, 100}` |
| `time` | `CqlValue::Time(i64)` | `{:time, integer()}` | `{:time, 3600000000000}` |
| `timestamp` | `CqlValue::Timestamp(i64)` | `{:timestamp, integer()}` | `{:timestamp, 1693000000000}` |
| `timeuuid` | `CqlValue::Timeuuid(CqlTimeuuid)` | `{:timeuuid, binary()}` | `{:timeuuid, uuid_bin}` |
| `tinyint` | `CqlValue::TinyInt(i8)` | `{:tinyint, integer()}` | `{:tinyint, 7}` |
| `uuid` | `CqlValue::Uuid(Uuid)` | `{:uuid, binary()}` | `{:uuid, uuid_bin}` |
| `varint` | `CqlValue::Varint(CqlVarint)` | `{:varint, binary()}` | `{:varint, "12345678901234567890"}` |
| `list<T>` | `CqlValue::List(Vec<CqlValue>)` | `{:list, list()}` | `{:list, [v1, v2]}` |
| `set<T>` | `CqlValue::Set(Vec<CqlValue>)` | `{:set, list()}` | `{:set, [v1, v2]}` |
| `map<K, V>` | `CqlValue::Map(Vec<(K, V)>)` | `{:map, list({k, v})}` | `{:map, [{k1, v1}]}` |
| `tuple<...>` | `CqlValue::Tuple(Vec<CqlValue>)` | `{:tuple, list()}` | `{:tuple, [v1, v2]}` |
| `UDT` | `CqlValue::UserDefinedType(...)` | `{:udt, %UserDefinedType{}}` | `{:udt, %UserDefinedType{}}` |
| `Empty` / `Null` | `CqlValue::Empty` | `{:null, nil}` | `nil` / `{:null, nil}` |

## Large Numbers & Decimals
- Decimal values are represented as string binaries (e.g. `{:decimal, "123.45"}`) backed by Rust's `bigdecimal 0.4` to maintain arbitrary precision without loss during BEAM float conversion.
- Varint values are represented as base-10 string binaries (e.g. `{:varint, "12345678901234567890"}`) backed by Rust's `num-bigint 0.4`.

## Raw Binary Queries
For extreme performance or specialized serializers, `Session.query_raw/3` and `Session.execute_raw/3` skip decoding CQL values into tagged Elixir terms and return `ExScylla.Types.QueryResultRaw` holding native memory references to row buffers.
