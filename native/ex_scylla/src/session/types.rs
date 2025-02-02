use crate::errors::ScyllaError;
use crate::prepared_statement::types::PreparedStatementResource;
use crate::query::types::QueryResource;
use crate::utils::*;
use rustler::types::Atom;
use rustler::{
    Binary, Decoder, Encoder, Env, NewBinary, NifResult, NifStruct, NifTaggedEnum, NifTuple,
    NifUntaggedEnum, ResourceArc, Term,
};
use scylla::batch::BatchStatement;
use scylla::deserialize::row::ColumnIterator;
use scylla::deserialize::{DeserializationError, DeserializeRow, DeserializeValue, FrameSlice, TypeCheckError};
use scylla::frame::response::result::{ColumnSpec, ColumnType};
use scylla::frame::value::CqlDuration;
use scylla::query::Query;
use scylla::serialize::writers::CellWriter;
use scylla::transport::{PagingState, PagingStateResponse};
use scylla::QueryResult;
use scylla::Session;
use scylla::routing::Token;
use std::convert::TryInto;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::ops::Deref;
use uuid::{Bytes, Uuid};
use scylla::serialize::value::SerializeValue;
use scylla::serialize::writers::WrittenCellProof;
use scylla::serialize::SerializationError;

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

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.Token"]
pub struct ScyllaToken {
    pub value: i64,
}

impl From<Token> for ScyllaToken {
    fn from(t: Token) -> Self {
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

#[derive(NifTaggedEnum)]
pub enum ScyllaPageState {
    Start,
    PageState(Vec<u8>),
}

impl ToRust<PagingState> for ScyllaPageState {
    fn r(self) -> PagingState {
        match self {
            ScyllaPageState::Start => PagingState::start(),
            ScyllaPageState::PageState(bytes) => PagingState::new_from_raw_bytes(bytes),
        }
    }
}

impl From<PagingState> for ScyllaPageState {
    fn from(ps: PagingState) -> Self {
        ps.as_bytes_slice()
        .map(|b| ScyllaPageState::PageState(b.to_vec()))
        .unwrap_or(ScyllaPageState::Start)
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

#[derive(NifTaggedEnum)]
pub enum ScyllaPagingStateResponse {
    HasMorePages { state: ScyllaPageState },
    NoMorePages,
}

impl From<PagingStateResponse> for ScyllaPagingStateResponse {
    fn from(psr: PagingStateResponse) -> Self {
        match psr {
            PagingStateResponse::HasMorePages { state } => ScyllaPagingStateResponse::HasMorePages {
                state: ScyllaPageState::from(state)
            },
            PagingStateResponse::NoMorePages => ScyllaPagingStateResponse::NoMorePages,
        }
    }
}

impl ToElixir<ScyllaPagingStateResponse> for PagingStateResponse {
    fn ex(self) -> ScyllaPagingStateResponse {
        ScyllaPagingStateResponse::from(self)
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
    //pub col_specs: Option<Vec<ScyllaColumnSpec>>,
    pub rows_byte_size: usize,
    pub rows_num: usize,
}

impl ToElixir<ScyllaQueryResult> for QueryResult {
    fn ex(self) -> ScyllaQueryResult {
        let mut res = ScyllaQueryResult {
            rows: None,
            warnings: self.warnings().map(|w| w.to_string()).collect(),
            tracing_id: self.tracing_id().map(|b| b.into()),
            rows_byte_size: 0,
            rows_num: 0,
        };
        if self.is_rows() {
            let qr = self.into_rows_result().unwrap();
            res.rows = Some(qr.rows::<ScyllaRow>().unwrap().into_iter().collect::<Result<Vec<_>, _>>().unwrap());
            res.rows_byte_size = qr.rows_bytes_size();
            res.rows_num = qr.rows_num();
        }
        res
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.ColumnSpec"]
pub struct ScyllaColumnSpec {
    pub table_spec: ScyllaTableSpec,
    pub name: String,
    pub typ: ScyllaColumnType,
}

to_elixir!(ColumnSpec<'_>, ScyllaColumnSpec, |qs: ColumnSpec<'_>| {
    ScyllaColumnSpec {
        table_spec: ScyllaTableSpec {
            ks_name: qs.table_spec().ks_name().to_string(),
            table_name: qs.table_spec().table_name().to_string(),
        },
        name: qs.name().to_string(),
        typ: qs.typ().into(),
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
    pub int_val: rustler::BigInt,
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

impl From<&ScyllaIpAddr> for IpAddr {
    fn from(sia: &ScyllaIpAddr) -> Self {
        match sia {
            ScyllaIpAddr::IPv4(v4) => IpAddr::V4(Ipv4Addr::new(v4.0, v4.1, v4.2, v4.3)),
            ScyllaIpAddr::IPv6(v6) => IpAddr::V6(Ipv6Addr::new(
                v6.0, v6.1, v6.2, v6.3, v6.4, v6.5, v6.6, v6.7,
            )),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.CustomValue"]
pub struct ScyllaCustomValue {
    pub name: String
}

#[derive(NifTaggedEnum, Debug)]
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
    Time(i64),
    Timeuuid(ScyllaBinary),
    Tuple(Vec<Option<ScyllaValue>>),
    Uuid(ScyllaBinary),
    Varint(String),
    Custom(ScyllaCustomValue),
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

impl From<&ColumnType<'_>> for ScyllaColumnType {
    fn from(ct: &ColumnType<'_>) -> Self {
        match ct {
            ColumnType::Custom(str) => ScyllaColumnType::Custom(str.to_string()),
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
            ColumnType::List(typ) => ScyllaColumnType::List(Box::new(ScyllaColumnType::from(&**typ))),
            ColumnType::Map(key, val) => ScyllaColumnType::Map((
                Box::new(ScyllaColumnType::from(&**key)),
                Box::new(ScyllaColumnType::from(&**val)),
            )),
            ColumnType::Set(typ) => ScyllaColumnType::Set(Box::new(ScyllaColumnType::from(&**typ))),
            ColumnType::UserDefinedType {
                type_name,
                keyspace,
                field_types,
            } => ScyllaColumnType::UserDefinedType(ScyllaUserDefinedColumnType {
                type_name: type_name.to_string(),
                keyspace: keyspace.to_string(),
                field_types: field_types
                    .into_iter()
                    .map(|(str, ct)| (str.to_string(), ct.into()))
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

impl<'frame, 'metadata> DeserializeValue<'frame, 'metadata> for ScyllaValue {
    fn type_check(_typ: &ColumnType) -> Result<(), TypeCheckError> {
        Ok(())
    }

    fn deserialize(
        typ: &'metadata ColumnType<'metadata>,
        v: Option<FrameSlice<'frame>>,
    ) -> Result<Self, DeserializationError> {
        match typ {
            ColumnType::Ascii => {
                let ascii = String::deserialize(typ, v)?;
                Ok(ScyllaValue::Ascii(ascii))
            }
            ColumnType::BigInt => {
                let bigint = i64::deserialize(typ, v)?;
                Ok(ScyllaValue::BigInt(bigint))
            }
            ColumnType::Blob => {
                let bytes = Vec::<u8>::deserialize(typ, v)?;
                Ok(ScyllaValue::Blob(bytes.into()))
            }
            ColumnType::Boolean => {
                let bool = bool::deserialize(typ, v)?;
                Ok(ScyllaValue::Boolean(bool))
            }
            ColumnType::Counter => {
                let counter = i64::deserialize(typ, v)?;
                Ok(ScyllaValue::Counter(counter))
            }
            ColumnType::Decimal => {
                let decimal = scylla::frame::value::CqlDecimal::deserialize(typ, v)?;
                let (bytes, scale) = decimal.as_signed_be_bytes_slice_and_exponent();
                let num_str = bytes.iter().fold(String::new(), |mut acc, b| {
                    acc.push_str(&format!("{:02x}", b));
                    acc
                });
                let num = i128::from_str_radix(&num_str, 16).unwrap();
                let decimal_str = if scale >= 0 {
                    format!("{}", num * 10_i128.pow(scale as u32))
                } else {
                    let div = 10_i128.pow((-scale) as u32);
                    let int_part = num / div;
                    let frac_part = num % div;
                    format!("{}.{}", int_part, frac_part.abs())
                };
                Ok(ScyllaValue::Decimal(decimal_str))
            }
            ColumnType::Double => {
                let double = f64::deserialize(typ, v)?;
                Ok(ScyllaValue::Double(double))
            }
            ColumnType::Float => {
                let float = f32::deserialize(typ, v)?;
                Ok(ScyllaValue::Float(float))
            }
            ColumnType::Int => {
                let int = i32::deserialize(typ, v)?;
                Ok(ScyllaValue::Int(int))
            }
            ColumnType::Timestamp => {
                let timestamp = i64::deserialize(typ, v)?;
                Ok(ScyllaValue::Timestamp(timestamp))
            }
            ColumnType::Uuid => {
                let uuid = uuid::Uuid::deserialize(typ, v)?;
                Ok(ScyllaValue::Uuid(uuid.as_bytes().to_vec().into()))
            }
            ColumnType::Varint => {
                let varint = scylla::frame::value::CqlVarint::deserialize(typ, v)?;
                let bytes = varint.into_signed_bytes_be();
                let num_str = bytes.iter().fold(String::new(), |mut acc, b| {
                    acc.push_str(&format!("{:02x}", b));
                    acc
                });
                Ok(ScyllaValue::Varint(num_str))
            }
            ColumnType::Timeuuid => {
                let timeuuid = scylla::frame::value::CqlTimeuuid::deserialize(typ, v)?;
                Ok(ScyllaValue::Timeuuid(timeuuid.as_bytes().to_vec().into()))
            }
            ColumnType::Inet => {
                let inet = IpAddr::deserialize(typ, v)?;
                Ok(ScyllaValue::Inet(inet.into()))
            }
            ColumnType::Date => {
                let date = scylla::frame::value::CqlDate::deserialize(typ, v)?;
                Ok(ScyllaValue::Date(date.0))
            }
            ColumnType::Time => {
                let time = scylla::frame::value::CqlTime::deserialize(typ, v)?;
                Ok(ScyllaValue::Time(time.0))
            }
            ColumnType::SmallInt => {
                let smallint = i16::deserialize(typ, v)?;
                Ok(ScyllaValue::SmallInt(smallint))
            }
            ColumnType::TinyInt => {
                let tinyint = i8::deserialize(typ, v)?;
                Ok(ScyllaValue::TinyInt(tinyint))
            }
            ColumnType::Duration => {
                let duration = scylla::frame::value::CqlDuration::deserialize(typ, v)?;
                Ok(ScyllaValue::Duration(ScyllaCqlDuration {
                    months: duration.months,
                    days: duration.days,
                    nanoseconds: duration.nanoseconds,
                }))
            }
            ColumnType::List(_item_type) => {
                let list = Vec::<ScyllaValue>::deserialize(typ, v)?;
                Ok(ScyllaValue::List(list))
            }
            ColumnType::Set(_item_type) => {
                let set = Vec::<ScyllaValue>::deserialize(typ, v)?;
                Ok(ScyllaValue::Set(set))
            }
            ColumnType::Map(_key_type, _value_type) => {
                let map = Vec::<(ScyllaValue, ScyllaValue)>::deserialize(typ, v)?;
                Ok(ScyllaValue::Map(map))
            }
            ColumnType::Tuple(_types) => {
                let tuple = Vec::<ScyllaValue>::deserialize(typ, v)?;
                let items = tuple.into_iter().map(|v| v.into()).collect();
                Ok(ScyllaValue::Tuple(items))
            }
            ColumnType::UserDefinedType { type_name, keyspace, field_types } => {
                let values = Vec::<ScyllaValue>::deserialize(typ, v)?;
                let fields = field_types.iter().zip(values).map(|((name, _), value)| {
                    (name.to_string(), Some(value))
                }).collect();
                Ok(ScyllaValue::UserDefinedType(ScyllaUserDefinedType {
                    type_name: type_name.to_string(),
                    keyspace: keyspace.to_string(),
                    fields
                }))
            }
            ColumnType::Custom(name) => {
                //let bytes = Vec::<u8>::deserialize(typ, v)?;
                Ok(ScyllaValue::Custom(ScyllaCustomValue {
                    name: name.to_string()
                }))
            }
            ColumnType::Text => {
                let text = String::deserialize(typ, v)?;
                Ok(ScyllaValue::Text(text))
            }
        }
    }

}

impl<'frame, 'metadata> DeserializeRow<'frame, 'metadata> for ScyllaRow {
    fn type_check(_specs: &[ColumnSpec<'_>]) -> Result<(), TypeCheckError> {
        // ScyllaRow can handle any column types, so we accept all
        Ok(())
    }

    fn deserialize(row: ColumnIterator<'frame, 'metadata>) -> Result<Self, DeserializationError> {
        let mut columns = Vec::with_capacity(row.size_hint().0);
        for col in row {
            let value = col?;
            let v = value.slice;
            let typ = value.spec.typ();
            let scylla_value = ScyllaValue::deserialize(typ, v)?;
            columns.push(Some(scylla_value));
        }
        Ok(ScyllaRow { columns })
    }
}

impl SerializeValue for ScyllaValue {
    fn serialize<'b>(
        &self,
        typ: &ColumnType,
        writer: CellWriter<'b>,
    ) -> Result<WrittenCellProof<'b>, SerializationError> {
        match self {
            ScyllaValue::Ascii(s) | ScyllaValue::Text(s) => {
                SerializeValue::serialize(s, typ, writer)
            }
            ScyllaValue::Boolean(b) => SerializeValue::serialize(b, typ, writer),
            ScyllaValue::Blob(bytes) => SerializeValue::serialize(&bytes.0, typ, writer),
            ScyllaValue::Counter(i) => SerializeValue::serialize(i, typ, writer),
            ScyllaValue::Decimal(s) => SerializeValue::serialize(s, typ, writer),
            ScyllaValue::Date(d) => {
                let cql_date = scylla::frame::value::CqlDate(*d);
                SerializeValue::serialize(&cql_date, typ, writer)
            },
            ScyllaValue::Double(d) => SerializeValue::serialize(d, typ, writer),
            ScyllaValue::Duration(d) => {
                let cql_duration = CqlDuration {
                    months: d.months,
                    days: d.days,
                    nanoseconds: d.nanoseconds,
                };
                SerializeValue::serialize(&cql_duration, typ, writer)
            },
            ScyllaValue::Empty => SerializeValue::serialize(&Option::<i32>::None, typ, writer),
            ScyllaValue::Float(f) => SerializeValue::serialize(f, typ, writer),
            ScyllaValue::Int(i) => SerializeValue::serialize(i, typ, writer),
            ScyllaValue::BigInt(i) => SerializeValue::serialize(i, typ, writer),
            ScyllaValue::Timestamp(ts) => SerializeValue::serialize(ts, typ, writer),
            ScyllaValue::Inet(addr) => {
                let ip: std::net::IpAddr = addr.into();
                SerializeValue::serialize(&ip, typ, writer)
            },
            ScyllaValue::List(items) => SerializeValue::serialize(items, typ, writer),
            ScyllaValue::Map(pairs) => SerializeValue::serialize(pairs, typ, writer),
            ScyllaValue::Set(items) => SerializeValue::serialize(items, typ, writer),
            ScyllaValue::UserDefinedType(udt) => {
                let values: Vec<Option<&ScyllaValue>> = udt.fields.iter()
                    .map(|(_, v)| v.as_ref())
                    .collect();
                SerializeValue::serialize(&values, typ, writer)
            },
            ScyllaValue::SmallInt(i) => SerializeValue::serialize(i, typ, writer),
            ScyllaValue::TinyInt(i) => SerializeValue::serialize(i, typ, writer),
            ScyllaValue::Time(t) => SerializeValue::serialize(t, typ, writer),
            ScyllaValue::Timeuuid(uuid) => SerializeValue::serialize(&uuid.0, typ, writer),
            ScyllaValue::Tuple(items) => SerializeValue::serialize(items, typ, writer),
            ScyllaValue::Uuid(uuid) => SerializeValue::serialize(&uuid.0, typ, writer),
            ScyllaValue::Varint(v) => SerializeValue::serialize(v, typ, writer),
            ScyllaValue::Custom(_) => SerializeValue::serialize(&Option::<i32>::None, typ, writer),
        }
    }
}
