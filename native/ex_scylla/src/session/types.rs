use crate::prepared_statement::types::PreparedStatementResource;
use crate::query::types::QueryResource;
use crate::utils::*;
use rustler::{
    Binary, Decoder, Encoder, Env, Error, NewBinary, NifResult, NifStruct, NifTaggedEnum,
    NifUntaggedEnum, NifTuple, ResourceArc, Term,
};
use scylla::client::session::Session;
use scylla::response::query_result::QueryResult;
use scylla::response::PagingStateResponse;
use scylla::statement::unprepared::Statement as Query;
use scylla::value::{CqlValue, CqlDuration, Counter};
use scylla_cql::frame::response::result::{ColumnType, ColumnSpec, NativeType, CollectionType};
use crate::errors::ScyllaError;

pub struct SessionResource(pub Session);

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.QueryResult"]
pub struct ScyllaQueryResult {
    pub rows: Option<Vec<ScyllaRow>>,
    pub warnings: Vec<String>,
    pub tracing_id: Option<ScyllaBinary>,
    pub paging_state: Option<ScyllaBinary>,
    pub serialized_size: usize,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Row"]
pub struct ScyllaRow {
    pub columns: Vec<Option<ScyllaValue>>,
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

impl Into<Query> for ScyllaQuery {
    fn into(self) -> Query {
        match self {
            Self::String(q) => q.into(),
            Self::QueryResource(q) => q.0.to_owned().into(),
        }
    }
}

impl ToElixir<ScyllaQueryResult> for QueryResult {
    fn ex(self) -> ScyllaQueryResult {
        let warnings = self.warnings().map(|s| s.to_string()).collect();
        let tracing_id = self.tracing_id().map(|id| ScyllaBinary(id.as_bytes().to_vec()));
        
        match self.into_rows_result() {
            Ok(rows_res) => {
                 let rows = rows_res.rows::<scylla::value::Row>().ok().map(|iter| {
                     iter.map(|r| {
                         let row = r.expect("Row deserialization failed");
                         ScyllaRow { columns: row.columns.into_iter().map(|c| c.map(|c| c.into())).collect() }
                     }).collect()
                 });
                 ScyllaQueryResult {
                     rows,
                     warnings,
                     tracing_id,
                     paging_state: None,
                     serialized_size: 0,
                 }
            },
            Err(_) => {
                ScyllaQueryResult {
                    rows: None,
                    warnings,
                    tracing_id,
                    paging_state: None,
                    serialized_size: 0,
                }
            }
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

impl From<CqlValue> for ScyllaValue {
    fn from(cv: CqlValue) -> Self {
        match cv {
            CqlValue::Ascii(ascii) => ScyllaValue::Ascii(ascii),
            CqlValue::Boolean(bool) => ScyllaValue::Boolean(bool),
            CqlValue::Blob(blob) => ScyllaValue::Blob(ScyllaBinary(blob)),
            CqlValue::Counter(counter) => ScyllaValue::Counter(counter.0),
            CqlValue::Decimal(decimal) => ScyllaValue::Decimal(format!("{:?}", decimal)),
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
            CqlValue::List(v) => ScyllaValue::List(v.into_iter().map(|cv| cv.into()).collect()),
            CqlValue::Map(v) => {
                ScyllaValue::Map(v.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            CqlValue::Set(v) => ScyllaValue::Set(v.into_iter().map(|cv| cv.into()).collect()),
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
                    .map(|(f, v)| (f, v.map(|v| v.into())))
                    .collect(),
            }),
            CqlValue::SmallInt(i16) => ScyllaValue::SmallInt(i16),
            CqlValue::TinyInt(i8) => ScyllaValue::TinyInt(i8),
            CqlValue::Time(d) => ScyllaValue::Time(d.0 as u64),
            CqlValue::Timeuuid(uuid) => ScyllaValue::Timeuuid(ScyllaBinary(uuid.as_bytes().to_vec())),
            CqlValue::Tuple(t) => {
                ScyllaValue::Tuple(t.into_iter().map(|v| v.map(|v| v.into())).collect())
            }
            CqlValue::Uuid(uuid) => ScyllaValue::Uuid(ScyllaBinary(uuid.as_bytes().to_vec())),
            CqlValue::Varint(varint) => ScyllaValue::Varint(format!("{:?}", varint)),
            _ => ScyllaValue::Empty,
        }
    }
}

impl Into<CqlValue> for ScyllaValue {
    fn into(self) -> CqlValue {
        match self {
            ScyllaValue::Ascii(s) => CqlValue::Ascii(s),
            ScyllaValue::Boolean(b) => CqlValue::Boolean(b),
            ScyllaValue::Blob(b) => CqlValue::Blob(b.0),
            ScyllaValue::Counter(c) => CqlValue::Counter(Counter(c)),
            ScyllaValue::Int(i) => CqlValue::Int(i),
            ScyllaValue::BigInt(i) => CqlValue::BigInt(i),
            ScyllaValue::Text(s) => CqlValue::Text(s),
            _ => CqlValue::Empty,
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
impl<'a> Encoder for ScyllaPageState {
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
        Ok(ScyllaPageState(scylla::response::PagingState::new_from_raw_bytes(c.to_vec())))
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
    Ascii,
    Boolean,
    Blob,
    Counter,
    Date,
    Decimal,
    Double,
    Duration,
    Float,
    Int,
    BigInt,
    Text,
    Timestamp,
    Inet,
    List(Box<ScyllaColumnType>),
    Map((Box<ScyllaColumnType>, Box<ScyllaColumnType>)),
    Set(Box<ScyllaColumnType>),
    UserDefinedType(ScyllaUserDefinedColumnType),
    SmallInt,
    TinyInt,
    Time,
    Timeuuid,
    Tuple(Vec<ScyllaColumnType>),
    Uuid,
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
                CollectionType::List(inner) => ScyllaColumnType::List(Box::new(ScyllaColumnType::from(*inner))),
                CollectionType::Set(inner) => ScyllaColumnType::Set(Box::new(ScyllaColumnType::from(*inner))),
                CollectionType::Map(k, v) => ScyllaColumnType::Map((Box::new(ScyllaColumnType::from(*k)), Box::new(ScyllaColumnType::from(*v)))),
                _ => ScyllaColumnType::Custom(format!("{:?}", typ)),
            },
            ColumnType::UserDefinedType { definition, .. } => ScyllaUserDefinedType_from_def(&definition),
            ColumnType::Tuple(vec) => {
                ScyllaColumnType::Tuple(vec.into_iter().map(|ct| ct.into()).collect())
            }
            _ => ScyllaColumnType::Custom("Unknown".to_string()),
        }
    }
}

fn ScyllaUserDefinedType_from_def(definition: &scylla_cql::frame::response::result::UserDefinedType) -> ScyllaColumnType {
    ScyllaColumnType::UserDefinedType(ScyllaUserDefinedColumnType {
        type_name: definition.name.to_string(),
        keyspace: definition.keyspace.to_string(),
        field_types: definition.field_types
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

impl Encoder for Box<ScyllaColumnType> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (**self).encode(env)
    }
}

impl<'a> Decoder<'a> for Box<ScyllaColumnType> {
    fn decode(_term: Term<'a>) -> NifResult<Self> {
        Err(Error::Atom("not_implemented"))
    }
}

use scylla_cql::serialize::writers::{WrittenCellProof, CellWriter};
use scylla_cql::serialize::value::SerializeValue;
use scylla_cql::serialize::SerializationError;

pub struct ScyllaTerm<'a>(pub Term<'a>);

impl<'a> SerializeValue for ScyllaTerm<'a> {
    fn serialize<'b>(
        &self,
        typ: &ColumnType,
        writer: CellWriter<'b>,
    ) -> Result<WrittenCellProof<'b>, SerializationError> {
        let term = self.0;
        match typ {
            ColumnType::Native(nt) => match nt {
                NativeType::Ascii | NativeType::Text => {
                    let s: String = term.decode().map_err(|_| SerializationError::new(ScyllaError::parse("Expected string")))?;
                    s.serialize(typ, writer)
                }
                NativeType::Int => {
                    let i: i32 = term.decode().map_err(|_| SerializationError::new(ScyllaError::parse("Expected i32")))?;
                    i.serialize(typ, writer)
                }
                NativeType::BigInt => {
                    let i: i64 = term.decode().map_err(|_| SerializationError::new(ScyllaError::parse("Expected i64")))?;
                    i.serialize(typ, writer)
                }
                NativeType::Boolean => {
                    let b: bool = term.decode().map_err(|_| SerializationError::new(ScyllaError::parse("Expected boolean")))?;
                    b.serialize(typ, writer)
                }
                NativeType::Blob => {
                    let b: Binary = term.decode().map_err(|_| SerializationError::new(ScyllaError::parse("Expected binary")))?;
                    b.as_slice().serialize(typ, writer)
                }
                _ => Err(SerializationError::new(ScyllaError::parse(&format!("Unsupported native type for lazy serialization: {:?}", nt)))),
            },
            _ => Err(SerializationError::new(ScyllaError::parse(&format!("Unsupported type for lazy serialization: {:?}", typ)))),
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum ScyllaBatchStatement {
    String(String),
    QueryResource(ResourceArc<QueryResource>),
    PreparedStatement(ResourceArc<PreparedStatementResource>),
}

impl Into<scylla::statement::batch::BatchStatement> for ScyllaBatchStatement {
    fn into(self) -> scylla::statement::batch::BatchStatement {
        match self {
            Self::String(q) => q.as_str().into(),
            Self::QueryResource(q) => q.0.to_owned().into(),
            Self::PreparedStatement(ps) => ps.0.to_owned().into(),
        }
    }
}

impl ToElixir<ResourceArc<SessionResource>> for Session {
    fn ex(self) -> ResourceArc<SessionResource> {
        ResourceArc::new(SessionResource(self))
    }
}
