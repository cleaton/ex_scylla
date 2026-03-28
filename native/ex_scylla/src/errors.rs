use crate::utils::*;
use rustler::types::Atom;
use rustler::{NifTaggedEnum, NifTuple};
pub use scylla::errors::{BadKeyspaceName, BadQuery, DbError, NewSessionError, ExecutionError as QueryError, TranslationError, RequestAttemptError};
pub use scylla::statement::prepared::PartitionKeyError;
pub use scylla::serialize::SerializationError as SerializeValuesError;

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
    RequestTimeout(String),
    TranslationError(ScyllaTranslationError),
}

to_elixir!(QueryError, ScyllaQueryError, |qe: QueryError| {
    match qe {
        QueryError::LastAttemptError(rae) => {
            match rae {
                RequestAttemptError::DbError(dbe, msg) => ScyllaQueryError::DbError(dbe.ex_with_msg(msg)),
                _ => ScyllaQueryError::IoError(rae.to_string()),
            }
        },
        QueryError::BadQuery(bq) => ScyllaQueryError::BadQuery(bq.ex()),
        _ => ScyllaQueryError::IoError(qe.to_string()),
    }
});

// Update ExScylla.Types.Errors.TranslationError on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaTranslationError {
    NoRuleForAddress(String),
    InvalidAddressInRule(String),
}

to_elixir!(
    TranslationError,
    ScyllaTranslationError,
    |te: TranslationError| {
        let msg = te.to_string();
        match te {
            TranslationError::InvalidAddressInRule { .. } => {
                ScyllaTranslationError::InvalidAddressInRule(msg)
            }
            TranslationError::NoRuleForAddress(_) => ScyllaTranslationError::NoRuleForAddress(msg),
            _ => ScyllaTranslationError::NoRuleForAddress(msg),
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

impl ScyllaDbError {
    fn from_db_error(dbe: DbError, msg: String) -> Self {
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
            _ => ScyllaDbError::Other(msg),
        }
    }
}

trait DbErrorExt {
    fn ex_with_msg(self, msg: String) -> ScyllaDbError;
}
impl DbErrorExt for DbError {
    fn ex_with_msg(self, msg: String) -> ScyllaDbError {
        ScyllaDbError::from_db_error(self, msg)
    }
}

// Update ExScylla.Types.Errors.BadQuery on change
#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaBadQuery {
    SerializeValuesError(ScyllaSerializeValuesError),
    ValuesTooLongForKey(String),
    BadKeyspaceName(ScyllaBadKeyspaceName),
    Other(String),
}

to_elixir!(BadQuery, ScyllaBadQuery, |bq: BadQuery| {
    let msg = bq.to_string();
    match bq {
        BadQuery::SerializationError(e) => ScyllaBadQuery::SerializeValuesError(ScyllaSerializeValuesError::ParseError(e.to_string())),
        BadQuery::ValuesTooLongForKey(_, _) => ScyllaBadQuery::ValuesTooLongForKey(msg),
        _ => ScyllaBadQuery::Other(msg),
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
        ScyllaSerializeValuesError::ParseError(sve.to_string())
    }
);

#[derive(NifTaggedEnum, Debug)]
pub enum ScyllaNewSessionError {
    FailedToResolveAnyHostname(String),
    EmptyKnownNodesList(String),
    DbError(ScyllaDbError),
    BadQuery(ScyllaBadQuery),
    IoError(String),
    ProtocolError(String),
    InvalidMessage(String),
    TimeoutError(String),
    TooManyOrphanedStreamIds(String),
    UnableToAllocStreamId(String),
    RequestTimeout(String),
    TranslationError(ScyllaTranslationError),
}
to_elixir!(NewSessionError, ScyllaNewSessionError, |nse: NewSessionError| {
    match nse {
        NewSessionError::FailedToResolveAnyHostname(_) => {
            ScyllaNewSessionError::FailedToResolveAnyHostname(nse.to_string())
        }
        NewSessionError::EmptyKnownNodesList => {
            ScyllaNewSessionError::EmptyKnownNodesList(nse.to_string())
        }
        _ => ScyllaNewSessionError::IoError(nse.to_string()),
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
            _ => ScyllaBadKeyspaceName::IllegalCharacter(msg),
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
        ScyllaPartitionKeyError::ValueTooLong(msg)
    }
);

#[derive(NifTuple, Debug)]
pub struct ScyllaError(Atom, String);

impl std::fmt::Display for ScyllaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScyllaError({:?}, {})", self.0, self.1)
    }
}

impl std::error::Error for ScyllaError {}

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
