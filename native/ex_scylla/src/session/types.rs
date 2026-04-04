use crate::prepared_statement::types::PreparedStatementResource;
use crate::query::types::QueryResource;
use crate::utils::*;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use rustler::{
    Binary, Decoder, Encoder, Env, NewBinary, NifResult, NifStruct, NifTaggedEnum, NifTuple,
    NifUntaggedEnum, ResourceArc, Term,
};
use scylla::client::session::Session;
use scylla::response::query_result::QueryResult;
use scylla::statement::unprepared::Statement as Query;
use scylla::value::{Counter, CqlDuration, CqlValue};
use scylla_cql::frame::response::result::{CollectionType, ColumnSpec, ColumnType, NativeType};
use std::str::FromStr;

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Metrics"]
pub struct ScyllaMetrics {
    pub errors_num: u64,
    pub queries_num: u64,
    pub errors_iter_num: u64,
    pub queries_iter_num: u64,
    pub retries_num: u64,
    pub mean_rate: f64,
    pub one_minute_rate: f64,
    pub five_minute_rate: f64,
    pub fifteen_minute_rate: f64,
    pub total_connections: u64,
    pub connection_timeouts: u64,
    pub request_timeouts: u64,
    pub latency_avg_ms: Option<u64>,
    pub latency_99_percentile_ms: Option<u64>,
}

impl From<&scylla::observability::metrics::Metrics> for ScyllaMetrics {
    fn from(m: &scylla::observability::metrics::Metrics) -> Self {
        ScyllaMetrics {
            errors_num: m.get_errors_num(),
            queries_num: m.get_queries_num(),
            errors_iter_num: m.get_errors_iter_num(),
            queries_iter_num: m.get_queries_iter_num(),
            retries_num: m.get_retries_num(),
            mean_rate: m.get_mean_rate(),
            one_minute_rate: m.get_one_minute_rate(),
            five_minute_rate: m.get_five_minute_rate(),
            fifteen_minute_rate: m.get_fifteen_minute_rate(),
            total_connections: m.get_total_connections(),
            connection_timeouts: m.get_connection_timeouts(),
            request_timeouts: m.get_request_timeouts(),
            latency_avg_ms: m.get_latency_avg_ms().ok(),
            latency_99_percentile_ms: m.get_latency_percentile_ms(99.0).ok(),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.TracingEvent"]
pub struct ScyllaTracingEvent {
    pub event_id: ScyllaBinary,
    pub activity: Option<String>,
    pub source: Option<ScyllaIpAddr>,
    pub source_elapsed: Option<i32>,
    pub thread: Option<String>,
}

impl From<scylla::observability::tracing::TracingEvent> for ScyllaTracingEvent {
    fn from(te: scylla::observability::tracing::TracingEvent) -> Self {
        ScyllaTracingEvent {
            event_id: ScyllaBinary(te.event_id.as_bytes().to_vec()),
            activity: te.activity,
            source: te.source.map(|s| s.into()),
            source_elapsed: te.source_elapsed,
            thread: te.thread,
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.TracingInfo"]
pub struct ScyllaTracingInfo {
    pub client: Option<ScyllaIpAddr>,
    pub command: Option<String>,
    pub coordinator: Option<ScyllaIpAddr>,
    pub duration: Option<i32>,
    pub parameters: Option<std::collections::HashMap<String, String>>,
    pub request: Option<String>,
    pub started_at: Option<i64>,
    pub events: Vec<ScyllaTracingEvent>,
}

impl From<scylla::observability::tracing::TracingInfo> for ScyllaTracingInfo {
    fn from(ti: scylla::observability::tracing::TracingInfo) -> Self {
        ScyllaTracingInfo {
            client: ti.client.map(|c| c.into()),
            command: ti.command,
            coordinator: ti.coordinator.map(|c| c.into()),
            duration: ti.duration,
            parameters: ti.parameters,
            request: ti.request,
            started_at: ti.started_at.map(|s| s.0),
            events: ti.events.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.NodeInfo"]
pub struct ScyllaNodeInfo {
    pub host_id: ScyllaBinary,
    pub address: ScyllaSocketAddr,
    pub datacenter: Option<String>,
    pub rack: Option<String>,
}

impl From<&scylla::cluster::Node> for ScyllaNodeInfo {
    fn from(node: &scylla::cluster::Node) -> Self {
        ScyllaNodeInfo {
            host_id: ScyllaBinary(node.host_id.as_bytes().to_vec()),
            address: ScyllaSocketAddr {
                addr: node.address.ip().into(),
                port: node.address.port(),
            },
            datacenter: node.datacenter.clone(),
            rack: node.rack.clone(),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.ClusterState"]
pub struct ScyllaClusterState {
    pub nodes: Vec<ScyllaNodeInfo>,
    pub keyspaces: Vec<String>,
}

impl From<&scylla::cluster::ClusterState> for ScyllaClusterState {
    fn from(cs: &scylla::cluster::ClusterState) -> Self {
        ScyllaClusterState {
            nodes: cs.get_nodes_info().iter().map(|n| (&**n).into()).collect(),
            keyspaces: cs.keyspaces_iter().map(|(k, _)| k.to_string()).collect(),
        }
    }
}

pub struct SessionResource(pub Session);
impl std::panic::RefUnwindSafe for SessionResource {}

pub struct ScyllaRawRowsResource(pub bytes::Bytes);

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.QueryResult"]
pub struct ScyllaQueryResult<'a> {
    pub rows: Option<Term<'a>>,
    pub rows_count: Option<usize>,
    pub column_types: Vec<ScyllaColumnType>,
    pub warnings: Vec<String>,
    pub tracing_id: Option<ScyllaBinary>,
    pub paging_state: Option<ScyllaBinary>,
    pub serialized_size: usize,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Token"]
pub struct ScyllaToken {
    pub value: i64,
}

impl From<scylla::routing::Token> for ScyllaToken {
    fn from(t: scylla::routing::Token) -> Self {
        ScyllaToken { value: t.value() }
    }
}

#[derive(NifTaggedEnum)]
pub enum ScyllaQuery {
    String(String),
    QueryResource(ResourceArc<QueryResource>),
}

impl From<ScyllaQuery> for Query {
    fn from(val: ScyllaQuery) -> Self {
        match val {
            ScyllaQuery::String(q) => q.into(),
            ScyllaQuery::QueryResource(q) => q.0.to_owned(),
        }
    }
}

impl<'a> ScyllaQueryResult<'a> {
    pub fn new(env: Env<'a>, qr: QueryResult) -> Self {
        let warnings = qr.warnings().map(|s| s.to_string()).collect();
        let tracing_id = qr
            .tracing_id()
            .map(|id| ScyllaBinary(id.as_bytes().to_vec()));

        match qr.into_rows_result() {
            Ok(rows_res) => {
                let column_types: Vec<ScyllaColumnType> = rows_res
                    .column_specs()
                    .iter()
                    .map(|cs| cs.typ().clone().into())
                    .collect();

                let rows_count = rows_res.raw_rows_with_metadata().rows_count();
                let raw_rows_bytes = rows_res.raw_rows_with_metadata().raw_rows().clone();
                let res_arc = rustler::ResourceArc::new(ScyllaRawRowsResource(raw_rows_bytes));
                let rows = Some(res_arc.make_binary(env, |r| r.0.as_ref()).to_term(env));

                ScyllaQueryResult {
                    rows,
                    rows_count: Some(rows_count),
                    column_types,
                    warnings,
                    tracing_id,
                    paging_state: None,
                    serialized_size: 0,
                }
            }
            Err(_) => ScyllaQueryResult {
                rows: None,
                rows_count: None,
                column_types: Vec::new(),
                warnings,
                tracing_id,
                paging_state: None,
                serialized_size: 0,
            },
        }
    }
}

#[derive(Debug, NifTaggedEnum)]
pub enum ScyllaValue {
    Ascii(String),
    Boolean(bool),
    Blob(ScyllaBinary),
    Counter(i64),
    Decimal(String),
    Date(u32),
    Double(f64),
    Duration(ScyllaCqlDuration),
    Empty,
    Float(f32),
    Int(i32),
    BigInt(i64),
    Text(String),
    Timestamp(i64),
    Inet(ScyllaIpAddr),
    List(Vec<ScyllaValue>),
    Map(Vec<(ScyllaValue, ScyllaValue)>),
    Set(Vec<ScyllaValue>),
    UserDefinedType(ScyllaUserDefinedType),
    SmallInt(i16),
    TinyInt(i8),
    Time(u64),
    Timeuuid(ScyllaBinary),
    Tuple(Vec<Option<ScyllaValue>>),
    Uuid(ScyllaBinary),
    Varint(String),
}

impl std::convert::TryFrom<CqlValue> for ScyllaValue {
    type Error = rustler::Error;

    fn try_from(cv: CqlValue) -> Result<Self, Self::Error> {
        Ok(match cv {
            CqlValue::Ascii(ascii) => ScyllaValue::Ascii(ascii),
            CqlValue::Boolean(bool) => ScyllaValue::Boolean(bool),
            CqlValue::Blob(blob) => ScyllaValue::Blob(ScyllaBinary(blob)),
            CqlValue::Counter(counter) => ScyllaValue::Counter(counter.0),
            CqlValue::Decimal(decimal) => {
                let (bi, scale) = decimal.into_signed_be_bytes_and_exponent();
                let bd = BigDecimal::from((BigInt::from_signed_bytes_be(&bi), scale as i64));
                ScyllaValue::Decimal(bd.to_string())
            }
            CqlValue::Date(date) => ScyllaValue::Date(date.0),
            CqlValue::Double(f64) => ScyllaValue::Double(f64),
            CqlValue::Duration(duration) => ScyllaValue::Duration(ScyllaCqlDuration {
                months: duration.months,
                days: duration.days,
                nanoseconds: duration.nanoseconds,
            }),
            CqlValue::Empty => ScyllaValue::Empty,
            CqlValue::Float(f32) => ScyllaValue::Float(f32),
            CqlValue::Int(i32) => ScyllaValue::Int(i32),
            CqlValue::BigInt(i64) => ScyllaValue::BigInt(i64),
            CqlValue::Text(text) => ScyllaValue::Text(text),
            CqlValue::Timestamp(d) => ScyllaValue::Timestamp(d.0),
            CqlValue::Inet(ipaddr) => ScyllaValue::Inet(ipaddr.into()),
            CqlValue::List(v) => ScyllaValue::List(
                v.into_iter()
                    .map(|cv| cv.try_into())
                    .collect::<Result<Vec<_>, rustler::Error>>()?,
            ),
            CqlValue::Map(v) => ScyllaValue::Map(
                v.into_iter()
                    .map(|(k, v)| Ok::<_, rustler::Error>((k.try_into()?, v.try_into()?)))
                    .collect::<Result<Vec<_>, rustler::Error>>()?,
            ),
            CqlValue::Set(v) => ScyllaValue::Set(
                v.into_iter()
                    .map(|cv| cv.try_into())
                    .collect::<Result<Vec<_>, rustler::Error>>()?,
            ),
            CqlValue::UserDefinedType {
                keyspace,
                name,
                fields,
                ..
            } => ScyllaValue::UserDefinedType(ScyllaUserDefinedType {
                keyspace,
                type_name: name,
                fields: fields
                    .into_iter()
                    .map(|(f, v)| {
                        Ok::<_, rustler::Error>((
                            f,
                            match v {
                                Some(v) => Some(v.try_into()?),
                                None => None,
                            },
                        ))
                    })
                    .collect::<Result<Vec<_>, rustler::Error>>()?,
            }),
            CqlValue::SmallInt(i16) => ScyllaValue::SmallInt(i16),
            CqlValue::TinyInt(i8) => ScyllaValue::TinyInt(i8),
            CqlValue::Time(d) => ScyllaValue::Time(d.0 as u64),
            CqlValue::Timeuuid(uuid) => {
                ScyllaValue::Timeuuid(ScyllaBinary(uuid.as_bytes().to_vec()))
            }
            CqlValue::Tuple(t) => ScyllaValue::Tuple(
                t.into_iter()
                    .map(|v| match v {
                        Some(val) => Ok::<_, rustler::Error>(Some(val.try_into()?)),
                        None => Ok(None),
                    })
                    .collect::<Result<Vec<_>, rustler::Error>>()?,
            ),
            CqlValue::Uuid(uuid) => ScyllaValue::Uuid(ScyllaBinary(uuid.as_bytes().to_vec())),
            CqlValue::Varint(varint) => {
                let bi = BigInt::from_signed_bytes_be(&varint.into_signed_bytes_be());
                ScyllaValue::Varint(bi.to_string())
            }
            _ => return Err(rustler::Error::Term(Box::new("unsupported_cql_value"))),
        })
    }
}

impl From<ScyllaValue> for CqlValue {
    fn from(val: ScyllaValue) -> Self {
        match val {
            ScyllaValue::Ascii(s) => CqlValue::Ascii(s),
            ScyllaValue::Boolean(b) => CqlValue::Boolean(b),
            ScyllaValue::Blob(b) => CqlValue::Blob(b.0),
            ScyllaValue::Counter(c) => CqlValue::Counter(Counter(c)),
            ScyllaValue::Decimal(decimal) => BigDecimal::from_str(&decimal)
                .map(|bd| {
                    let (bi, scale) = bd.into_bigint_and_exponent();
                    CqlValue::Decimal(
                        scylla::value::CqlDecimal::from_signed_be_bytes_and_exponent(
                            bi.to_signed_bytes_be(),
                            scale as i32,
                        ),
                    )
                })
                .unwrap_or(CqlValue::Empty),
            ScyllaValue::Date(u32) => CqlValue::Date(scylla::value::CqlDate(u32)),
            ScyllaValue::Double(f64) => CqlValue::Double(f64),
            ScyllaValue::Duration(cd) => CqlValue::Duration(CqlDuration {
                months: cd.months,
                days: cd.days,
                nanoseconds: cd.nanoseconds,
            }),
            ScyllaValue::Empty => CqlValue::Empty,
            ScyllaValue::Float(f32) => CqlValue::Float(f32),
            ScyllaValue::Int(i32) => CqlValue::Int(i32),
            ScyllaValue::BigInt(i64) => CqlValue::BigInt(i64),
            ScyllaValue::Text(text) => CqlValue::Text(text),
            ScyllaValue::Timestamp(i64) => CqlValue::Timestamp(scylla::value::CqlTimestamp(i64)),
            ScyllaValue::Inet(ipaddr) => {
                let ip: std::net::IpAddr = match ipaddr {
                    ScyllaIpAddr::IPv4(v4) => {
                        std::net::IpAddr::V4(std::net::Ipv4Addr::new(v4.0, v4.1, v4.2, v4.3))
                    }
                    ScyllaIpAddr::IPv6(v6) => std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                        v6.0, v6.1, v6.2, v6.3, v6.4, v6.5, v6.6, v6.7,
                    )),
                };
                CqlValue::Inet(ip)
            }
            ScyllaValue::List(v) => {
                let values = v.into_iter().map(|sv| sv.into()).collect();
                CqlValue::List(values)
            }
            ScyllaValue::Map(v) => {
                let values = v.into_iter().map(|(k, v)| (k.into(), v.into())).collect();
                CqlValue::Map(values)
            }
            ScyllaValue::Set(v) => {
                let values = v.into_iter().map(|sv| sv.into()).collect();
                CqlValue::Set(values)
            }
            ScyllaValue::UserDefinedType(sudt) => {
                let fields = sudt
                    .fields
                    .into_iter()
                    .map(|(t, v)| (t, v.map(|v| v.into())))
                    .collect();
                CqlValue::UserDefinedType {
                    keyspace: sudt.keyspace,
                    name: sudt.type_name,
                    fields,
                }
            }
            ScyllaValue::SmallInt(i16) => CqlValue::SmallInt(i16),
            ScyllaValue::TinyInt(i8) => CqlValue::TinyInt(i8),
            ScyllaValue::Time(u64) => CqlValue::Time(scylla::value::CqlTime(u64 as i64)),
            ScyllaValue::Timeuuid(u) => {
                if u.0.len() == 16 {
                    let mut slice: [u8; 16] = Default::default();
                    slice.copy_from_slice(u.0.as_slice());
                    CqlValue::Timeuuid(uuid::Uuid::from_bytes(slice).into())
                } else {
                    CqlValue::Empty
                }
            }
            ScyllaValue::Tuple(v) => {
                let values = v.into_iter().map(|sv| sv.map(|sv| sv.into())).collect();
                CqlValue::Tuple(values)
            }
            ScyllaValue::Uuid(u) => {
                if u.0.len() == 16 {
                    let mut slice: [u8; 16] = Default::default();
                    slice.copy_from_slice(u.0.as_slice());
                    CqlValue::Uuid(uuid::Uuid::from_bytes(slice))
                } else {
                    CqlValue::Empty
                }
            }
            ScyllaValue::Varint(varint) => BigInt::from_str(&varint)
                .map(|bi| {
                    CqlValue::Varint(scylla::value::CqlVarint::from_signed_bytes_be(
                        bi.to_signed_bytes_be(),
                    ))
                })
                .unwrap_or(CqlValue::Empty),
        }
    }
}

#[derive(Debug)]
pub struct ScyllaBinary(pub Vec<u8>);
impl From<Vec<u8>> for ScyllaBinary {
    fn from(v: Vec<u8>) -> Self {
        ScyllaBinary(v)
    }
}
impl From<bytes::Bytes> for ScyllaBinary {
    fn from(v: bytes::Bytes) -> Self {
        ScyllaBinary(v.to_vec())
    }
}
impl From<uuid::Uuid> for ScyllaBinary {
    fn from(u: uuid::Uuid) -> Self {
        ScyllaBinary(u.as_bytes().to_vec())
    }
}
impl<'a> Encoder for ScyllaBinary {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let mut nb = NewBinary::new(env, self.0.len());
        nb.as_mut_slice().copy_from_slice(&self.0);
        nb.into()
    }
}
impl<'a> Decoder<'a> for ScyllaBinary {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let b: Binary<'a> = term.decode()?;
        Ok(ScyllaBinary(b.as_slice().to_vec()))
    }
}

impl ToElixir<String> for String {
    fn ex(self) -> String {
        self
    }
}

pub struct ScyllaUuid(pub uuid::Uuid);
impl Encoder for ScyllaUuid {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let bytes = self.0.as_bytes();
        let mut nb = NewBinary::new(env, bytes.len());
        nb.as_mut_slice().copy_from_slice(bytes);
        nb.into()
    }
}
impl ToElixir<ScyllaUuid> for uuid::Uuid {
    fn ex(self) -> ScyllaUuid {
        ScyllaUuid(self)
    }
}

pub struct ScyllaPageState(pub scylla::response::PagingState);
impl Encoder for ScyllaPageState {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let bytes = self.0.as_bytes_slice().map(|arc| &**arc).unwrap_or(&[]);
        let mut nb = NewBinary::new(env, bytes.len());
        nb.as_mut_slice().copy_from_slice(bytes);
        nb.into()
    }
}
impl<'a> Decoder<'a> for ScyllaPageState {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let c: Binary<'a> = term.decode()?;
        Ok(ScyllaPageState(
            scylla::response::PagingState::new_from_raw_bytes(c.to_vec()),
        ))
    }
}

impl ToRust<Option<scylla::response::PagingState>> for Option<ScyllaPageState> {
    fn r(self) -> Option<scylla::response::PagingState> {
        self.map(|s| s.0)
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.CqlDuration"]
pub struct ScyllaCqlDuration {
    pub months: i32,
    pub days: i32,
    pub nanoseconds: i64,
}

#[derive(Debug, NifUntaggedEnum)]
pub enum ScyllaIpAddr {
    IPv6(IPv6),
    IPv4(IPv4),
}
#[derive(Debug, NifTuple)]
pub struct IPv6(u16, u16, u16, u16, u16, u16, u16, u16);
#[derive(Debug, NifTuple)]
pub struct IPv4(u8, u8, u8, u8);

impl From<std::net::IpAddr> for ScyllaIpAddr {
    fn from(ia: std::net::IpAddr) -> Self {
        match ia {
            std::net::IpAddr::V4(v4) => {
                let [a, b, c, d] = v4.octets();
                ScyllaIpAddr::IPv4(IPv4(a, b, c, d))
            }
            std::net::IpAddr::V6(v6) => {
                let [a, b, c, d, e, f, g, h] = v6.segments();
                ScyllaIpAddr::IPv6(IPv6(a, b, c, d, e, f, g, h))
            }
        }
    }
}

#[derive(Debug, NifTuple)]
pub struct ScyllaSocketAddr {
    pub addr: ScyllaIpAddr,
    pub port: u16,
}

impl From<ScyllaSocketAddr> for std::net::SocketAddr {
    fn from(ssa: ScyllaSocketAddr) -> Self {
        match ssa.addr {
            ScyllaIpAddr::IPv4(v4) => std::net::SocketAddr::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(v4.0, v4.1, v4.2, v4.3)),
                ssa.port,
            ),
            ScyllaIpAddr::IPv6(v6) => std::net::SocketAddr::new(
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                    v6.0, v6.1, v6.2, v6.3, v6.4, v6.5, v6.6, v6.7,
                )),
                ssa.port,
            ),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.UserDefinedType"]
pub struct ScyllaUserDefinedType {
    pub type_name: String,
    pub keyspace: String,
    pub fields: Vec<(String, Option<ScyllaValue>)>,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.TableSpec"]
pub struct ScyllaTableSpec {
    pub ks_name: String,
    pub table_name: String,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.ColumnSpec"]
pub struct ScyllaColumnSpec {
    pub table_spec: ScyllaTableSpec,
    pub name: String,
    pub typ: ScyllaColumnType,
}

#[derive(Debug, NifTaggedEnum)]
pub enum ScyllaColumnType {
    Custom(String),
    #[rustler(rename = "ascii")]
    Ascii,
    #[rustler(rename = "boolean")]
    Boolean,
    #[rustler(rename = "blob")]
    Blob,
    #[rustler(rename = "counter")]
    Counter,
    #[rustler(rename = "date")]
    Date,
    #[rustler(rename = "decimal")]
    Decimal,
    #[rustler(rename = "double")]
    Double,
    #[rustler(rename = "duration")]
    Duration,
    #[rustler(rename = "float")]
    Float,
    #[rustler(rename = "int")]
    Int,
    #[rustler(rename = "big_int")]
    BigInt,
    #[rustler(rename = "text")]
    Text,
    #[rustler(rename = "timestamp")]
    Timestamp,
    #[rustler(rename = "inet")]
    Inet,
    #[rustler(rename = "list")]
    List(Box<ScyllaColumnType>),
    #[rustler(rename = "map")]
    Map((Box<ScyllaColumnType>, Box<ScyllaColumnType>)),
    #[rustler(rename = "set")]
    Set(Box<ScyllaColumnType>),
    #[rustler(rename = "user_defined_type")]
    UserDefinedType(ScyllaUserDefinedColumnType),
    #[rustler(rename = "small_int")]
    SmallInt,
    #[rustler(rename = "tiny_int")]
    TinyInt,
    #[rustler(rename = "time")]
    Time,
    #[rustler(rename = "timeuuid")]
    Timeuuid,
    #[rustler(rename = "tuple")]
    Tuple(Vec<ScyllaColumnType>),
    #[rustler(rename = "uuid")]
    Uuid,
    #[rustler(rename = "varint")]
    Varint,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.UserDefinedColumnType"]
pub struct ScyllaUserDefinedColumnType {
    pub type_name: String,
    pub keyspace: String,
    pub field_types: Vec<(String, ScyllaColumnType)>,
}

impl From<ColumnType<'_>> for ScyllaColumnType {
    fn from(ct: ColumnType<'_>) -> Self {
        match ct {
            ColumnType::Native(nt) => match nt {
                NativeType::Ascii => ScyllaColumnType::Ascii,
                NativeType::Boolean => ScyllaColumnType::Boolean,
                NativeType::Blob => ScyllaColumnType::Blob,
                NativeType::Counter => ScyllaColumnType::Counter,
                NativeType::Date => ScyllaColumnType::Date,
                NativeType::Decimal => ScyllaColumnType::Decimal,
                NativeType::Double => ScyllaColumnType::Double,
                NativeType::Duration => ScyllaColumnType::Duration,
                NativeType::Float => ScyllaColumnType::Float,
                NativeType::Int => ScyllaColumnType::Int,
                NativeType::BigInt => ScyllaColumnType::BigInt,
                NativeType::Text => ScyllaColumnType::Text,
                NativeType::Timestamp => ScyllaColumnType::Timestamp,
                NativeType::Inet => ScyllaColumnType::Inet,
                NativeType::SmallInt => ScyllaColumnType::SmallInt,
                NativeType::TinyInt => ScyllaColumnType::TinyInt,
                NativeType::Time => ScyllaColumnType::Time,
                NativeType::Timeuuid => ScyllaColumnType::Timeuuid,
                NativeType::Uuid => ScyllaColumnType::Uuid,
                NativeType::Varint => ScyllaColumnType::Varint,
                _ => ScyllaColumnType::Custom(format!("{:?}", nt)),
            },
            ColumnType::Collection { typ, .. } => match typ {
                CollectionType::List(inner) => {
                    ScyllaColumnType::List(Box::new(ScyllaColumnType::from(*inner)))
                }
                CollectionType::Set(inner) => {
                    ScyllaColumnType::Set(Box::new(ScyllaColumnType::from(*inner)))
                }
                CollectionType::Map(k, v) => ScyllaColumnType::Map((
                    Box::new(ScyllaColumnType::from(*k)),
                    Box::new(ScyllaColumnType::from(*v)),
                )),
                _ => ScyllaColumnType::Custom(format!("{:?}", typ)),
            },
            ColumnType::UserDefinedType { definition, .. } => scylla_udt_from_def(&definition),
            ColumnType::Tuple(vec) => {
                ScyllaColumnType::Tuple(vec.into_iter().map(|ct| ct.into()).collect())
            }
            _ => ScyllaColumnType::Custom("Unknown".to_string()),
        }
    }
}

fn scylla_udt_from_def(
    definition: &scylla_cql::frame::response::result::UserDefinedType,
) -> ScyllaColumnType {
    ScyllaColumnType::UserDefinedType(ScyllaUserDefinedColumnType {
        type_name: definition.name.to_string(),
        keyspace: definition.keyspace.to_string(),
        field_types: definition
            .field_types
            .iter()
            .map(|(str, ct)| (str.to_string(), ct.clone().into()))
            .collect(),
    })
}

impl ToElixir<ScyllaColumnSpec> for ColumnSpec<'_> {
    fn ex(self) -> ScyllaColumnSpec {
        ScyllaColumnSpec {
            table_spec: ScyllaTableSpec {
                ks_name: self.table_spec().ks_name().to_string(),
                table_name: self.table_spec().table_name().to_string(),
            },
            name: self.name().to_string(),
            typ: self.typ().clone().into(),
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum ScyllaBatchStatement {
    String(String),
    QueryResource(ResourceArc<QueryResource>),
    PreparedStatement(ResourceArc<PreparedStatementResource>),
}

impl From<ScyllaBatchStatement> for scylla::statement::batch::BatchStatement {
    fn from(val: ScyllaBatchStatement) -> Self {
        match val {
            ScyllaBatchStatement::String(q) => q.as_str().into(),
            ScyllaBatchStatement::QueryResource(q) => q.0.to_owned().into(),
            ScyllaBatchStatement::PreparedStatement(ps) => ps.0.to_owned().into(),
        }
    }
}

impl ToElixir<ResourceArc<SessionResource>> for Session {
    fn ex(self) -> ResourceArc<SessionResource> {
        ResourceArc::new(SessionResource(self))
    }
}
