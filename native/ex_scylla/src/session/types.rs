use crate::errors::ScyllaError;
use crate::prepared_statement::types::PreparedStatementResource;
use crate::query::types::QueryResource;
use crate::utils::*;
use bigdecimal::BigDecimal;
use chrono;
use num_bigint::BigInt;
use rustler::types::Atom;
use rustler::{
    Binary, Decoder, Encoder, Env, Error, NewBinary, NifResult, NifStruct, NifTaggedEnum, NifTuple,
    NifUntaggedEnum, ResourceArc, Term,
};
use rustler_bigint::BigInt as RustlerBigInt;
use scylla::batch::BatchStatement;
use scylla::frame::response::result::{ColumnSpec, ColumnType, CqlValue, Row};
use scylla::frame::value::Counter;
use scylla::frame::value::CqlDuration;
use scylla::query::Query;
use scylla::Session;
use scylla::{BatchResult, QueryResult};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::ops::Deref;
use std::str::FromStr;
use uuid::{Bytes, Uuid};

pub struct SessionResource(pub Session);

#[derive(NifUntaggedEnum)]
pub enum ScyllaBatchStatement {
    String(String),
    QueryResource(ResourceArc<QueryResource>),
    PreparedStatement(ResourceArc<PreparedStatementResource>),
}

impl Into<BatchStatement> for ScyllaBatchStatement {
    fn into(self) -> BatchStatement {
        match self {
            Self::String(q) => q.as_str().into(),
            Self::QueryResource(q) => q.0.to_owned().into(),
            Self::PreparedStatement(ps) => ps.0.to_owned().into(),
        }
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

pub struct ScyllaPageState(scylla::Bytes);
impl<'a> Encoder for ScyllaPageState {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let mut nb = NewBinary::new(env, self.0.len());
        nb.as_mut_slice().write_all(&self.0).unwrap();
        nb.into()
    }
}
impl<'a> Decoder<'a> for ScyllaPageState {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let c: Binary<'a> = term.decode()?;
        Ok(ScyllaPageState(scylla::Bytes::from(c.to_vec())))
    }
}
#[derive(Debug)]
pub struct ScyllaBinary(Vec<u8>);
impl Deref for ScyllaBinary {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> Encoder for ScyllaBinary {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let mut nb = NewBinary::new(env, self.0.len());
        nb.as_mut_slice().write_all(&self.0).unwrap();
        nb.into()
    }
}
impl<'a> Decoder<'a> for ScyllaBinary {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let c: Binary<'a> = term.decode()?;
        Ok(ScyllaBinary(c.to_vec()))
    }
}
impl From<Vec<u8>> for ScyllaBinary {
    fn from(v: Vec<u8>) -> Self {
        ScyllaBinary(v)
    }
}
impl From<scylla::Bytes> for ScyllaBinary {
    fn from(v: scylla::Bytes) -> Self {
        ScyllaBinary(v.to_vec())
    }
}
impl From<uuid::Uuid> for ScyllaBinary {
    fn from(v: uuid::Uuid) -> Self {
        ScyllaBinary(v.as_bytes().to_vec())
    }
}

impl ToRust<Option<scylla::Bytes>> for Option<ScyllaPageState> {
    fn r(self) -> Option<scylla::Bytes> {
        self.map(|s| s.0)
    }
}
pub struct ScyllaUuuid(Uuid);
impl<'a> Encoder for ScyllaUuuid {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        let sb: ScyllaBinary = self.0.into();
        sb.encode(env)
    }
}
impl<'a> Decoder<'a> for ScyllaUuuid {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let c: ScyllaBinary = term.decode()?;
        let bytes: Bytes =
            c.0.try_into()
                .map_err(|_| ScyllaError::parse("Invalid byte size for uuid"))?;
        Ok(ScyllaUuuid(Uuid::from_bytes(bytes)))
    }
}

to_elixir!(Uuid, ScyllaUuuid, |uuid: Uuid| { ScyllaUuuid(uuid) });

to_elixir!(Session, ResourceArc<SessionResource>, |session| {
    ResourceArc::new(SessionResource(session))
});

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.QueryResult"]
pub struct ScyllaQueryResult {
    pub rows: Option<Vec<ScyllaRow>>,
    pub warnings: Vec<String>,
    pub tracing_id: Option<ScyllaBinary>,
    pub paging_state: Option<ScyllaBinary>,
    pub col_specs: Option<Vec<ScyllaColumnSpec>>,
}

to_elixir!(QueryResult, ScyllaQueryResult, |qr: QueryResult| {
    ScyllaQueryResult {
        rows: qr
            .rows
            .map(|rows| rows.into_iter().map(|row| row.ex()).collect()),
        warnings: qr.warnings,
        tracing_id: qr.tracing_id.map(|b| b.into()),
        paging_state: qr.paging_state.map(|b| b.into()),
        col_specs: Some(qr.col_specs.into_iter().map(|x| x.ex()).collect()),
    }
});

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.BatchResult"]
pub struct ScyllaBatchResult {
    pub warnings: Vec<String>,
    pub tracing_id: Option<ScyllaBinary>,
}

to_elixir!(BatchResult, ScyllaBatchResult, |br: BatchResult| {
    ScyllaBatchResult {
        warnings: br.warnings,
        tracing_id: br.tracing_id.map(|b| b.into()),
    }
});

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.ColumnSpec"]
pub struct ScyllaColumnSpec {
    pub table_spec: ScyllaTableSpec,
    pub name: String,
    pub typ: ScyllaColumnType,
}

to_elixir!(ColumnSpec, ScyllaColumnSpec, |qs: ColumnSpec| {
    ScyllaColumnSpec {
        table_spec: ScyllaTableSpec {
            ks_name: qs.table_spec.ks_name,
            table_name: qs.table_spec.table_name,
        },
        name: qs.name,
        typ: qs.typ.into(),
    }
});

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.TableSpec"]
pub struct ScyllaTableSpec {
    pub ks_name: String,
    pub table_name: String,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.BigDecimal"]
pub struct ScyllaBigDecimal {
    pub int_val: RustlerBigInt,
    // A positive scale means a negative power of 10
    pub scale: i64,
}
#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.CqlDuration"]
pub struct ScyllaCqlDuration {
    pub months: i32,
    pub days: i32,
    pub nanoseconds: i64,
}
impl From<CqlDuration> for ScyllaCqlDuration {
    fn from(cd: CqlDuration) -> Self {
        ScyllaCqlDuration {
            months: cd.months,
            days: cd.days,
            nanoseconds: cd.nanoseconds,
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Duration"]
pub struct ScyllaDuration {
    secs: i64,
    nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
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

impl From<IpAddr> for ScyllaIpAddr {
    fn from(ia: IpAddr) -> Self {
        match ia {
            IpAddr::V4(v4) => {
                let [a, b, c, d] = v4.octets();
                ScyllaIpAddr::IPv4(IPv4(a, b, c, d))
            }
            IpAddr::V6(v6) => {
                let [a, b, c, d, e, f, g, h] = v6.segments();
                ScyllaIpAddr::IPv6(IPv6(a, b, c, d, e, f, g, h))
            }
        }
    }
}

impl From<ScyllaIpAddr> for IpAddr {
    fn from(sia: ScyllaIpAddr) -> Self {
        match sia {
            ScyllaIpAddr::IPv4(v4) => IpAddr::V4(Ipv4Addr::new(v4.0, v4.1, v4.2, v4.3)),
            ScyllaIpAddr::IPv6(v6) => IpAddr::V6(Ipv6Addr::new(
                v6.0, v6.1, v6.2, v6.3, v6.4, v6.5, v6.6, v6.7,
            )),
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
    /// Days since -5877641-06-23 i.e. 2^31 days before unix epoch
    /// Can be converted to chrono::NaiveDate (-262145-1-1 to 262143-12-31) using as_date
    Date(u32),
    Double(f64),
    Duration(ScyllaCqlDuration),
    Empty,
    Float(f32),
    Int(i32),
    BigInt(i64),
    Text(String),
    /// Milliseconds since unix epoch
    Timestamp(i64),
    Inet(ScyllaIpAddr),
    List(Vec<ScyllaValue>),
    Map(Vec<(ScyllaValue, ScyllaValue)>),
    Set(Vec<ScyllaValue>),
    UserDefinedType(ScyllaUserDefinedType),
    SmallInt(i16),
    TinyInt(i8),
    /// Nanoseconds since midnight
    Time(u64),
    Timeuuid(ScyllaBinary),
    Tuple(Vec<Option<ScyllaValue>>),
    Uuid(ScyllaBinary),
    Varint(String),
}

impl TryFrom<ScyllaValue> for CqlValue {
    type Error = ScyllaError;
    fn try_from(sv: ScyllaValue) -> Result<Self, ScyllaError> {
        match sv {
            ScyllaValue::Ascii(str) => Ok(CqlValue::Ascii(str)),
            ScyllaValue::Boolean(bool) => Ok(CqlValue::Boolean(bool)),
            ScyllaValue::Blob(blob) => Ok(CqlValue::Blob(blob.0)),
            ScyllaValue::Counter(i64) => Ok(CqlValue::Counter(Counter(i64))),
            ScyllaValue::Decimal(decimal) => {
                let bd = BigDecimal::from_str(&decimal)
                    .map_err(|_| ScyllaError::parse("Failed to parse decimal"))?;
                Ok(CqlValue::Decimal(bd))
            }
            ScyllaValue::Date(u32) => Ok(CqlValue::Date(u32)),
            ScyllaValue::Double(f64) => Ok(CqlValue::Double(f64)),
            ScyllaValue::Duration(cd) => Ok(CqlValue::Duration(CqlDuration {
                months: cd.months,
                days: cd.days,
                nanoseconds: cd.nanoseconds,
            })),
            ScyllaValue::Empty => Ok(CqlValue::Empty),
            ScyllaValue::Float(f32) => Ok(CqlValue::Float(f32)),
            ScyllaValue::Int(i32) => Ok(CqlValue::Int(i32)),
            ScyllaValue::BigInt(i64) => Ok(CqlValue::BigInt(i64)),
            ScyllaValue::Text(text) => Ok(CqlValue::Text(text)),
            ScyllaValue::Timestamp(i64) => {
                Ok(CqlValue::Timestamp(chrono::Duration::milliseconds(i64)))
            }
            ScyllaValue::Inet(ipaddr) => {
                let ip: IpAddr = match ipaddr {
                    ScyllaIpAddr::IPv4(v4) => IpAddr::V4(Ipv4Addr::new(v4.0, v4.1, v4.2, v4.3)),
                    ScyllaIpAddr::IPv6(v6) => IpAddr::V6(Ipv6Addr::new(
                        v6.0, v6.1, v6.2, v6.3, v6.4, v6.5, v6.6, v6.7,
                    )),
                };
                Ok(CqlValue::Inet(ip))
            }
            ScyllaValue::List(v) => {
                let values = v
                    .into_iter()
                    .map(|sv| sv.try_into())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CqlValue::List(values))
            }
            ScyllaValue::Map(v) => {
                let values = v
                    .into_iter()
                    .map(|(k, v)| {
                        let kcv: Result<CqlValue, _> = k.try_into();
                        let vcv: Result<CqlValue, _> = v.try_into();
                        match (kcv, vcv) {
                            (Ok(kcv), Ok(vcv)) => Ok((kcv, vcv)),
                            (Err(err), _) => Err(err),
                            (_, Err(err)) => Err(err),
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CqlValue::Map(values))
            }
            ScyllaValue::Set(v) => {
                let values = v
                    .into_iter()
                    .map(|sv| sv.try_into())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CqlValue::Set(values))
            }
            ScyllaValue::UserDefinedType(sudt) => {
                let fields = sudt
                    .fields
                    .into_iter()
                    .map(|(t, v)| {
                        let v: Result<Option<CqlValue>, _> = v.map(|sv| sv.try_into()).transpose();
                        match v {
                            Ok(v) => Ok((t, v)),
                            Err(err) => Err(err),
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CqlValue::UserDefinedType {
                    keyspace: sudt.keyspace,
                    type_name: sudt.type_name,
                    fields,
                })
            }
            ScyllaValue::SmallInt(i16) => Ok(CqlValue::SmallInt(i16)),
            ScyllaValue::TinyInt(i8) => Ok(CqlValue::TinyInt(i8)),
            ScyllaValue::Time(u64) => Ok(CqlValue::Time(chrono::Duration::milliseconds(
                u64.try_into().unwrap(),
            ))),
            ScyllaValue::Timeuuid(u) => {
                if u.len() == 16 {
                    let mut slice: [u8; 16] = Default::default();
                    slice.copy_from_slice(u.as_slice());
                    Ok(CqlValue::Timeuuid(uuid::Uuid::from_bytes(slice)))
                } else {
                    Err(ScyllaError::parse("invalid uuid byte length"))
                }
            }
            ScyllaValue::Tuple(v) => {
                let values = v
                    .into_iter()
                    .map(|sv| sv.map(|sv| sv.try_into()).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(CqlValue::Tuple(values))
            }
            ScyllaValue::Uuid(u) => {
                if u.len() != 16 {
                    return Err(ScyllaError::parse("invalid uuid byte length"));
                }
                let mut slice: [u8; 16] = Default::default();
                slice.copy_from_slice(u.as_slice());
                Ok(CqlValue::Uuid(uuid::Uuid::from_bytes(slice)))
            }
            ScyllaValue::Varint(varint) => {
                let bi = BigInt::from_str(&varint)
                    .map_err(|_| ScyllaError::parse("unable to parse varint"))?;
                Ok(CqlValue::Varint(bi))
            }
        }
    }
}

impl From<CqlValue> for ScyllaValue {
    fn from(cv: CqlValue) -> Self {
        match cv {
            CqlValue::Ascii(ascii) => ScyllaValue::Ascii(ascii),
            CqlValue::Boolean(bool) => ScyllaValue::Boolean(bool),
            CqlValue::Blob(blob) => ScyllaValue::Blob(blob.into()),
            CqlValue::Counter(counter) => ScyllaValue::Counter(counter.0),
            CqlValue::Decimal(decimal) => ScyllaValue::Decimal(decimal.to_string()),
            CqlValue::Date(u32) => ScyllaValue::Date(u32),
            CqlValue::Double(f64) => ScyllaValue::Double(f64),
            CqlValue::Duration(duration) => ScyllaValue::Duration(duration.into()),
            CqlValue::Empty => ScyllaValue::Empty,
            CqlValue::Float(f32) => ScyllaValue::Float(f32),
            CqlValue::Int(i32) => ScyllaValue::Int(i32),
            CqlValue::BigInt(i64) => ScyllaValue::BigInt(i64),
            CqlValue::Text(text) => ScyllaValue::Text(text),
            CqlValue::Timestamp(d) => ScyllaValue::Timestamp(d.num_milliseconds()),
            CqlValue::Inet(ipaddr) => ScyllaValue::Inet(ipaddr.into()),
            CqlValue::List(v) => ScyllaValue::List(v.into_iter().map(|cv| cv.into()).collect()),
            CqlValue::Map(v) => {
                ScyllaValue::Map(v.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            CqlValue::Set(v) => ScyllaValue::Set(v.into_iter().map(|cv| cv.into()).collect()),
            CqlValue::UserDefinedType {
                keyspace,
                type_name,
                fields,
            } => ScyllaValue::UserDefinedType(ScyllaUserDefinedType {
                keyspace,
                type_name,
                fields: fields
                    .into_iter()
                    .map(|(f, v)| (f, v.map(|v| v.into())))
                    .collect(),
            }),
            CqlValue::SmallInt(i16) => ScyllaValue::SmallInt(i16),
            CqlValue::TinyInt(i8) => ScyllaValue::TinyInt(i8),
            CqlValue::Time(d) => ScyllaValue::Time(
                d.num_nanoseconds()
                    .expect("Nanoseconds since midnight should never overflow")
                    .try_into()
                    .unwrap(),
            ),
            CqlValue::Timeuuid(uuid) => ScyllaValue::Timeuuid(uuid.into()),
            CqlValue::Tuple(t) => {
                ScyllaValue::Tuple(t.into_iter().map(|v| v.map(|v| v.into())).collect())
            }
            CqlValue::Uuid(uuid) => ScyllaValue::Uuid(uuid.into()),
            CqlValue::Varint(varint) => ScyllaValue::Varint(varint.to_string()),
        }
    }
}

impl ToRust<Result<Vec<CqlValue>, Error>> for Vec<ScyllaValue> {
    fn r(self) -> Result<Vec<CqlValue>, Error> {
        self.into_iter()
            .map(|sv| sv.try_into())
            .collect::<Result<Vec<CqlValue>, ScyllaError>>()
            .map_err(|se| Error::from(se))
    }
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

impl From<ColumnType> for ScyllaColumnType {
    fn from(ct: ColumnType) -> Self {
        match ct {
            ColumnType::Custom(str) => ScyllaColumnType::Custom(str),
            ColumnType::Ascii => ScyllaColumnType::Ascii,
            ColumnType::Boolean => ScyllaColumnType::Boolean,
            ColumnType::Blob => ScyllaColumnType::Blob,
            ColumnType::Counter => ScyllaColumnType::Counter,
            ColumnType::Date => ScyllaColumnType::Date,
            ColumnType::Decimal => ScyllaColumnType::Decimal,
            ColumnType::Double => ScyllaColumnType::Double,
            ColumnType::Duration => ScyllaColumnType::Duration,
            ColumnType::Float => ScyllaColumnType::Float,
            ColumnType::Int => ScyllaColumnType::Int,
            ColumnType::BigInt => ScyllaColumnType::BigInt,
            ColumnType::Text => ScyllaColumnType::Text,
            ColumnType::Timestamp => ScyllaColumnType::Timestamp,
            ColumnType::Inet => ScyllaColumnType::Inet,
            ColumnType::List(typ) => ScyllaColumnType::List(Box::new(ScyllaColumnType::from(*typ))),
            ColumnType::Map(key, val) => ScyllaColumnType::Map((
                Box::new(ScyllaColumnType::from(*key)),
                Box::new(ScyllaColumnType::from(*val)),
            )),
            ColumnType::Set(typ) => ScyllaColumnType::Set(Box::new(ScyllaColumnType::from(*typ))),
            ColumnType::UserDefinedType {
                type_name,
                keyspace,
                field_types,
            } => ScyllaColumnType::UserDefinedType(ScyllaUserDefinedColumnType {
                type_name,
                keyspace,
                field_types: field_types
                    .into_iter()
                    .map(|(str, ct)| (str, ct.into()))
                    .collect(),
            }),
            ColumnType::SmallInt => ScyllaColumnType::SmallInt,
            ColumnType::TinyInt => ScyllaColumnType::TinyInt,
            ColumnType::Time => ScyllaColumnType::Time,
            ColumnType::Timeuuid => ScyllaColumnType::Timeuuid,
            ColumnType::Tuple(vec) => {
                ScyllaColumnType::Tuple(vec.into_iter().map(|ct| ct.into()).collect())
            }
            ColumnType::Uuid => ScyllaColumnType::Uuid,
            ColumnType::Varint => ScyllaColumnType::Varint,
        }
    }
}

impl Encoder for Box<ScyllaColumnType> {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        (**self).encode(env)
    }
}
impl<'a> Decoder<'a> for Box<ScyllaColumnType> {
    fn decode(term: Term) -> NifResult<Self> {
        let typ: ScyllaColumnType = term.decode()?;
        Ok(Box::new(typ))
    }
}

#[derive(NifTuple, Debug)]
pub struct ListColumnType(Atom, ScyllaColumnType);
#[derive(NifTuple, Debug)]
pub struct MapColumnType(Atom, ScyllaColumnType, ScyllaColumnType);
#[derive(NifTuple, Debug)]
pub struct TupleColumnType(Atom, Vec<ScyllaColumnType>);
#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.UserDefinedType"]
pub struct ScyllaUserDefinedType {
    pub type_name: String,
    pub keyspace: String,
    pub fields: Vec<(String, Option<ScyllaValue>)>,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.UserDefinedColumnType"]
pub struct ScyllaUserDefinedColumnType {
    pub type_name: String,
    pub keyspace: String,
    pub field_types: Vec<(String, ScyllaColumnType)>,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Row"]
pub struct ScyllaRow {
    pub columns: Vec<Option<ScyllaValue>>,
}

to_elixir!(Row, ScyllaRow, |r: Row| {
    ScyllaRow {
        columns: r
            .columns
            .into_iter()
            .map(|v| v.map(|v| v.into()))
            .collect::<Vec<_>>(),
    }
});
