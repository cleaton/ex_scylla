use crate::utils::*;
use rustler::types::Atom;
use rustler::{NifTaggedEnum, NifTuple};
use scylla::transport::errors::{BadKeyspaceName, BadQuery, DbError};
use scylla::transport::session::TranslationError;
use scylla::{
    prepared_statement::PartitionKeyError,
    transport::errors::{NewSessionError, QueryError},
};

rustler::atoms! {
    parse_value,
}

// Update ExScylla.Types.Errors.QueryError on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaQueryError {
    DbError(ScyllaDbError),
    BadQuery(ScyllaBadQuery),
    CqlRequestSerialization(String),
    BodyExtensionsParseError(String),
    EmptyPlan(String),
    CqlResultParseError(String),
    CqlErrorParseError(String),
    MetadataError(String),
    ConnectionPoolError(String),
    ProtocolError(String),
    TimeoutError(String),
    BrokenConnection(String),
    UnableToAllocStreamId(String),
    RequestTimeout(String),
    NextRowError(String),
    IntoLegacyQueryResultError(String),
    Unknown(String),
}

to_elixir!(QueryError, ScyllaQueryError, |qe: QueryError| {
    match qe {
        QueryError::DbError(dbe, msg) => ScyllaQueryError::DbError(dbe.ex()),
        QueryError::BadQuery(bq) => ScyllaQueryError::BadQuery(bq.ex()),
        QueryError::CqlRequestSerialization(err) => ScyllaQueryError::CqlRequestSerialization(err.to_string()),
        QueryError::BodyExtensionsParseError(err) => ScyllaQueryError::BodyExtensionsParseError(err.to_string()),
        QueryError::EmptyPlan => ScyllaQueryError::EmptyPlan(qe.to_string()),
        QueryError::CqlResultParseError(err) => ScyllaQueryError::CqlResultParseError(err.to_string()),
        QueryError::CqlErrorParseError(err) => ScyllaQueryError::CqlErrorParseError(err.to_string()),
        QueryError::MetadataError(err) => ScyllaQueryError::MetadataError(err.to_string()),
        QueryError::ConnectionPoolError(err) => ScyllaQueryError::ConnectionPoolError(err.to_string()),
        QueryError::ProtocolError(err) => ScyllaQueryError::ProtocolError(err.to_string()),
        QueryError::TimeoutError => ScyllaQueryError::TimeoutError(qe.to_string()),
        QueryError::BrokenConnection(err) => ScyllaQueryError::BrokenConnection(err.to_string()),
        QueryError::UnableToAllocStreamId => ScyllaQueryError::UnableToAllocStreamId(qe.to_string()),
        QueryError::RequestTimeout(msg) => ScyllaQueryError::RequestTimeout(msg),
        QueryError::NextRowError(err) => ScyllaQueryError::NextRowError(err.to_string()),
        #[allow(deprecated)]
        QueryError::IntoLegacyQueryResultError(err) => ScyllaQueryError::IntoLegacyQueryResultError(err.to_string()),
        _ => ScyllaQueryError::Unknown(qe.to_string()),
    }
});

// Update ExScylla.Types.Errors.TranslationError on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaTranslationError {
    NoRuleForAddress(String),
    InvalidAddressInRule(String),
    Unknown(String),
}

to_elixir!(
    TranslationError,
    ScyllaTranslationError,
    |te: TranslationError| {
        let msg = te.to_string();
        match te {
            TranslationError::NoRuleForAddress(addr) => ScyllaTranslationError::NoRuleForAddress(addr.to_string()),
            TranslationError::InvalidAddressInRule { translated_addr_str, reason } => 
                ScyllaTranslationError::InvalidAddressInRule(format!("Failed to parse translated address: {}, reason: {}", translated_addr_str, reason)),
            _ => ScyllaTranslationError::Unknown(te.to_string()),
        }
    }
);

// Update ExScylla.Types.Errors.DbError on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaDbError {
    SyntaxError(String),
    Invalid(String),
    AlreadyExists(String),
    FunctionFailure(String),
    AuthenticationError(String),
    Unauthorized(String),
    ConfigError(String),
    Unavailable(String),
    Overloaded(String),
    IsBootstrapping(String),
    TruncateError(String),
    ReadTimeout(String),
    WriteTimeout(String),
    ReadFailure(String),
    WriteFailure(String),
    Unprepared(String),
    ServerError(String),
    ProtocolError(String),
    RateLimitReached(String),
    Other(String),
}
to_elixir!(DbError, ScyllaDbError, |dbe: DbError| {
    let msg = dbe.to_string();
    match dbe {
        DbError::SyntaxError => ScyllaDbError::SyntaxError(msg),
        DbError::Invalid => ScyllaDbError::Invalid(msg),
        DbError::AlreadyExists { .. } => ScyllaDbError::AlreadyExists(msg),
        DbError::FunctionFailure { .. } => ScyllaDbError::FunctionFailure(msg),
        DbError::AuthenticationError => ScyllaDbError::AuthenticationError(msg),
        DbError::Unauthorized => ScyllaDbError::Unauthorized(msg),
        DbError::ConfigError => ScyllaDbError::ConfigError(msg),
        DbError::Unavailable { .. } => ScyllaDbError::Unavailable(msg),
        DbError::Overloaded => ScyllaDbError::Overloaded(msg),
        DbError::IsBootstrapping => ScyllaDbError::IsBootstrapping(msg),
        DbError::TruncateError => ScyllaDbError::TruncateError(msg),
        DbError::ReadTimeout { .. } => ScyllaDbError::ReadTimeout(msg),
        DbError::WriteTimeout { .. } => ScyllaDbError::WriteTimeout(msg),
        DbError::ReadFailure { .. } => ScyllaDbError::ReadFailure(msg),
        DbError::WriteFailure { .. } => ScyllaDbError::WriteFailure(msg),
        DbError::Unprepared { .. } => ScyllaDbError::Unprepared(msg),
        DbError::ServerError => ScyllaDbError::ServerError(msg),
        DbError::ProtocolError => ScyllaDbError::ProtocolError(msg),
        DbError::RateLimitReached { .. } => ScyllaDbError::RateLimitReached(msg),
        DbError::Other(_) => ScyllaDbError::Other(msg),
    }
});

// Update ExScylla.Types.Errors.BadQuery on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaBadQuery {
    SerializeValuesError(ScyllaSerializeValuesError),
    ValuesTooLongForKey(String),
    BadKeyspaceName(ScyllaBadKeyspaceName),
    Other(String),
}

to_elixir!(BadQuery, ScyllaBadQuery, |bq| {
    match bq {
        BadQuery::ValuesTooLongForKey(_, _) => ScyllaBadQuery::ValuesTooLongForKey(bq.to_string()),
        BadQuery::BadKeyspaceName(bkn) => ScyllaBadQuery::BadKeyspaceName(bkn.ex()),
        BadQuery::Other(msg) => ScyllaBadQuery::Other(msg),
        _ => ScyllaBadQuery::Other(bq.to_string()),
    }
});

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaSerializeValuesError {
    TooManyValues(String),
    MixingNamedAndNotNamedValues(String),
    ValueTooBig(String),
    ParseError(String),
}

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaNewSessionError {
    FailedToResolveAnyHostname(String),
    EmptyKnownNodesList(String),
    DbError(ScyllaDbError),
    BadQuery(ScyllaBadQuery),
    ProtocolError(String),
    TimeoutError(String),
    UnableToAllocStreamId(String),
    RequestTimeout(String),
    Unknown(String),
}

to_elixir!(NewSessionError, ScyllaNewSessionError, |nse| {
    match nse {
        NewSessionError::FailedToResolveAnyHostname(_) => {
            ScyllaNewSessionError::FailedToResolveAnyHostname(nse.to_string())
        }
        NewSessionError::EmptyKnownNodesList => {
            ScyllaNewSessionError::EmptyKnownNodesList(nse.to_string())
        }
        NewSessionError::DbError(dbe, _) => ScyllaNewSessionError::DbError(dbe.ex()),
        NewSessionError::BadQuery(bq) => ScyllaNewSessionError::BadQuery(bq.ex()),
        NewSessionError::ProtocolError(_) => ScyllaNewSessionError::ProtocolError(nse.to_string()),
        NewSessionError::TimeoutError => ScyllaNewSessionError::TimeoutError(nse.to_string()),
        NewSessionError::UnableToAllocStreamId => {
            ScyllaNewSessionError::UnableToAllocStreamId(nse.to_string())
        }
        NewSessionError::RequestTimeout(msg) => ScyllaNewSessionError::RequestTimeout(msg),
        _ => ScyllaNewSessionError::Unknown(nse.to_string()),
    }
});

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaBadKeyspaceName {
    Empty(String),
    TooLong(String),
    IllegalCharacter(String),
}

to_elixir!(
    BadKeyspaceName,
    ScyllaBadKeyspaceName,
    |bkn: BadKeyspaceName| {
        let msg = bkn.to_string();
        match bkn {
            BadKeyspaceName::Empty => ScyllaBadKeyspaceName::Empty(msg),
            BadKeyspaceName::TooLong(_, _) => ScyllaBadKeyspaceName::TooLong(msg),
            BadKeyspaceName::IllegalCharacter(_, _) => ScyllaBadKeyspaceName::IllegalCharacter(msg),
            _ => ScyllaBadKeyspaceName::IllegalCharacter(msg), // Catch-all for future variants
        }
    }
);

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaPartitionKeyError {
    PartitionKeyExtraction(String),
    TokenCalculation(String),
    Serialization(String),
    Unknown(String),
}

to_elixir!(
    PartitionKeyError,
    ScyllaPartitionKeyError,
    |pke: PartitionKeyError| {
        let msg = pke.to_string();
        match pke {
            PartitionKeyError::PartitionKeyExtraction(err) => ScyllaPartitionKeyError::PartitionKeyExtraction(err.to_string()),
            PartitionKeyError::TokenCalculation(err) => ScyllaPartitionKeyError::TokenCalculation(err.to_string()),
            PartitionKeyError::Serialization(err) => ScyllaPartitionKeyError::Serialization(err.to_string()),
            _ => ScyllaPartitionKeyError::Unknown(msg),
        }
    }
);

#[derive(NifTuple, Debug)]
pub struct ScyllaError(Atom, String);

impl ScyllaError {
    pub fn parse(msg: &str) -> ScyllaError {
        ScyllaError(parse_value(), msg.to_string())
    }
}

impl From<ScyllaError> for rustler::Error {
    fn from(se: ScyllaError) -> Self {
        rustler::Error::Term(Box::new(se))
    }
}

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaRowError {
    DeserializationError(String),
    TypeCheckError(String),
    QueryError(ScyllaQueryError),
}