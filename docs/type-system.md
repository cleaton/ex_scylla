# Type System & CQL Mapping

ExScylla provides bidirectional mapping between Apache Cassandra / ScyllaDB CQL types, Rust driver types (`scylla::value::CqlValue`), and Elixir terms.

## Data Type Mapping Reference

Bound values are tagged tuples (`{:tag, value}`), `nil` for NULL, or `:empty` for a
zero-length value. Typed result rows hold `{column_type, value}` per cell, or a bare
`nil` for NULL (and for an empty cell of a type with no empty form). Raw rows hold
the decoded value alone.

| CQL type | Rust driver (`CqlValue`) | Bound value | Decoded value |
| :--- | :--- | :--- | :--- |
| `ascii` | `CqlValue::Ascii` | `{:ascii, "abc"}` | `"abc"` |
| `text` / `varchar` | `CqlValue::Text` | `{:text, "abc"}` | `"abc"` |
| `bigint` | `CqlValue::BigInt` | `{:big_int, 123}` | `123` |
| `counter` | `CqlValue::Counter` | `{:counter, 1}` | `1` |
| `blob` | `CqlValue::Blob` | `{:blob, <<1, 2, 3>>}` | `<<1, 2, 3>>` |
| `boolean` | `CqlValue::Boolean` | `{:boolean, true}` | `true` |
| `date` | `CqlValue::Date` | `{:date, 2_147_501_898}` (days, 2^31 = epoch) | integer |
| `decimal` | `CqlValue::Decimal` | `{:decimal, "123.45"}` | `"123.45"` |
| `double` | `CqlValue::Double` | `{:double, 3.14159}` | float |
| `duration` | `CqlValue::Duration` | `{:duration, %CqlDuration{}}` | `%CqlDuration{}` |
| `float` | `CqlValue::Float` | `{:float, 2.5}` | float |
| `inet` | `CqlValue::Inet` | `{:inet, {127, 0, 0, 1}}` (IPv4 or IPv6 tuple) | IP tuple |
| `int` | `CqlValue::Int` | `{:int, 42}` | `42` |
| `smallint` | `CqlValue::SmallInt` | `{:small_int, 100}` | `100` |
| `tinyint` | `CqlValue::TinyInt` | `{:tiny_int, 7}` | `7` |
| `time` | `CqlValue::Time` | `{:time, 3_600_000_000_000}` (ns since midnight) | integer |
| `timestamp` | `CqlValue::Timestamp` | `{:timestamp, 1_693_000_000_000}` (ms) | integer |
| `uuid` | `CqlValue::Uuid` | `{:uuid, uuid}`: 16-byte binary or UUID string | hyphenated lowercase string |
| `timeuuid` | `CqlValue::Timeuuid` | `{:timeuuid, uuid}`: 16-byte binary or UUID string | hyphenated lowercase string |
| `varint` | `CqlValue::Varint` | `{:varint, "12345678901234567890"}` | string |
| `list<T>` | `CqlValue::List` | `{:list, [{:int, 1}]}` | list |
| `set<T>` | `CqlValue::Set` | `{:set, [{:int, 1}]}` | list |
| `map<K, V>` | `CqlValue::Map` | `{:map, [{{:text, "k"}, {:int, 1}}]}` | map |
| `tuple<...>` | `CqlValue::Tuple` | `{:tuple, [{:int, 1}, nil]}` | tuple |
| UDT | `CqlValue::UserDefinedType` | `{:user_defined_type, %UserDefinedType{}}` | map keyed by field name |
| NULL | `None` (bound as `Option<CqlValue>`) | `nil` | `nil` |
| empty value | `CqlValue::Empty` | `:empty` | `""` for text/ascii/blob; `{}`/`%{}` for tuple/UDT; `nil` otherwise |

Invalid input (an unknown tag, a UUID that is neither 16 bytes nor a UUID string, an
unparsable `varint`/`decimal` string) returns `{:error, {:parse_value, msg}}`.

## Large Numbers & Decimals
- Decimal values are represented as string binaries (e.g. `{:decimal, "123.45"}`) backed by Rust's `bigdecimal 0.4` to maintain arbitrary precision without loss during BEAM float conversion.
- Varint values are represented as base-10 string binaries (e.g. `{:varint, "12345678901234567890"}`) backed by Rust's `num-bigint 0.5`.

## Raw Binary Queries
For extreme performance or specialized serializers, `Session.query_raw/3` and `Session.execute_raw/3` skip decoding CQL values into tagged Elixir terms and return `ExScylla.Types.QueryResultRaw` holding native memory references to row buffers.
