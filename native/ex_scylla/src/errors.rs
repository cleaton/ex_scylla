use crate::utils::*;
use rustler::types::Atom;
use rustler::{NifTaggedEnum, NifTuple};
use scylla::frame::value::SerializeValuesError;
use scylla::transport::errors::{BadKeyspaceName, BadQuery, DbError};
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
    IoError(String),
    ProtocolError(String),
    InvalidMessage(String),
    TimeoutError(String),
    TooManyOrphanedStreamIds(String),
    UnableToAllocStreamId(String),
}
to_elixir!(QueryError, ScyllaQueryError, |qe: QueryError| {
    match qe {
        QueryError::DbError(dbe, _) => ScyllaQueryError::DbError(dbe.ex()),
        QueryError::BadQuery(bq) => ScyllaQueryError::BadQuery(bq.ex()),
        QueryError::IoError(_) => ScyllaQueryError::IoError(qe.to_string()),
        QueryError::ProtocolError(_) => ScyllaQueryError::ProtocolError(qe.to_string()),
        QueryError::InvalidMessage(_) => ScyllaQueryError::InvalidMessage(qe.to_string()),
        QueryError::TimeoutError => ScyllaQueryError::TimeoutError(qe.to_string()),
        QueryError::TooManyOrphanedStreamIds(_) => {
            ScyllaQueryError::TooManyOrphanedStreamIds(qe.to_string())
        }
        QueryError::UnableToAllocStreamId => {
            ScyllaQueryError::UnableToAllocStreamId(qe.to_string())
        }
    }
});

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
        DbError::Other(_) => ScyllaDbError::Other(msg),
    }
});

// Update ExScylla.Types.Errors.BadQuery on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaBadQuery {
    SerializeValuesError(ScyllaSerializeValuesError),
    ValueLenMismatch(String),
    ValuesTooLongForKey(String),
    BadKeyspaceName(ScyllaBadKeyspaceName),
}

to_elixir!(BadQuery, ScyllaBadQuery, |bq| {
    match bq {
        BadQuery::SerializeValuesError(e) => ScyllaBadQuery::SerializeValuesError(e.ex()),
        BadQuery::ValueLenMismatch(_, _) => ScyllaBadQuery::ValueLenMismatch(bq.to_string()),
        BadQuery::ValuesTooLongForKey(_, _) => ScyllaBadQuery::ValuesTooLongForKey(bq.to_string()),
        BadQuery::BadKeyspaceName(bkn) => ScyllaBadQuery::BadKeyspaceName(bkn.ex()),
    }
});

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaSerializeValuesError {
    TooManyValues(String),
    MixingNamedAndNotNamedValues(String),
    ValueTooBig(String),
    ParseError(String),
}

to_elixir!(
    SerializeValuesError,
    ScyllaSerializeValuesError,
    |sve: SerializeValuesError| {
        let msg = sve.to_string();
        match sve {
            SerializeValuesError::TooManyValues => ScyllaSerializeValuesError::TooManyValues(msg),
            SerializeValuesError::MixingNamedAndNotNamedValues => {
                ScyllaSerializeValuesError::MixingNamedAndNotNamedValues(msg)
            }
            SerializeValuesError::ValueTooBig(_) => ScyllaSerializeValuesError::ValueTooBig(msg),
            SerializeValuesError::ParseError => ScyllaSerializeValuesError::ParseError(msg),
        }
    }
);

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaNewSessionError {
    FailedToResolveAddress(String),
    EmptyKnownNodesList(String),
    DbError(ScyllaDbError),
    BadQuery(ScyllaBadQuery),
    IoError(String),
    ProtocolError(String),
    InvalidMessage(String),
    TimeoutError(String),
    TooManyOrphanedStreamIds(String),
    UnableToAllocStreamId(String),
}
to_elixir!(NewSessionError, ScyllaNewSessionError, |nse| {
    match nse {
        NewSessionError::FailedToResolveAddress(_) => {
            ScyllaNewSessionError::FailedToResolveAddress(nse.to_string())
        }
        NewSessionError::EmptyKnownNodesList => {
            ScyllaNewSessionError::EmptyKnownNodesList(nse.to_string())
        }
        NewSessionError::DbError(dbe, _) => ScyllaNewSessionError::DbError(dbe.ex()),
        NewSessionError::BadQuery(bq) => ScyllaNewSessionError::BadQuery(bq.ex()),
        NewSessionError::IoError(_) => ScyllaNewSessionError::IoError(nse.to_string()),
        NewSessionError::ProtocolError(_) => ScyllaNewSessionError::ProtocolError(nse.to_string()),
        NewSessionError::InvalidMessage(_) => {
            ScyllaNewSessionError::InvalidMessage(nse.to_string())
        }
        NewSessionError::TimeoutError => ScyllaNewSessionError::TimeoutError(nse.to_string()),
        NewSessionError::TooManyOrphanedStreamIds(_) => {
            ScyllaNewSessionError::TooManyOrphanedStreamIds(nse.to_string())
        }
        NewSessionError::UnableToAllocStreamId => {
            ScyllaNewSessionError::UnableToAllocStreamId(nse.to_string())
        }
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
        }
    }
);
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaPartitionKeyError {
    NoPkIndexValue(String),
    ValueTooLong(String),
}
to_elixir!(
    PartitionKeyError,
    ScyllaPartitionKeyError,
    |pke: PartitionKeyError| {
        let msg = pke.to_string();
        match pke {
            PartitionKeyError::NoPkIndexValue(_, _) => ScyllaPartitionKeyError::NoPkIndexValue(msg),
            PartitionKeyError::ValueTooLong(_) => ScyllaPartitionKeyError::ValueTooLong(msg),
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
