use rustler::{NifStruct, ResourceArc};
use scylla::{
    frame::response::result::PartitionKeyIndex,
    statement::prepared_statement::PreparedStatement,
};

use crate::{
    session::types::*,
    utils::{to_elixir, ToElixir},
};

use std::panic::RefUnwindSafe;

pub struct PreparedStatementResource(pub PreparedStatement);
impl RefUnwindSafe for PreparedStatementResource {}

to_elixir!(
    PreparedStatement,
    ResourceArc<PreparedStatementResource>,
    |ps: PreparedStatement| { ResourceArc::new(PreparedStatementResource(ps)) }
);

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.PreparedMetadata"]
pub struct ScyllaPreparedMetadata {
    pub col_count: usize,
    /// pk_indexes are sorted by `index` and can be reordered in partition key order
    /// using `sequence` field
    pub pk_indexes: Vec<ScyllaPartitionKeyIndex>,
    pub col_specs: Vec<ScyllaColumnSpec>,
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.PartitionKeyIndex"]
pub struct ScyllaPartitionKeyIndex {
    /// index in the serialized values
    pub index: u16,
    /// sequence number in partition key
    pub sequence: u16,
}

impl From<PartitionKeyIndex> for ScyllaPartitionKeyIndex {
    fn from(pki: PartitionKeyIndex) -> Self {
        ScyllaPartitionKeyIndex {
            index: pki.index,
            sequence: pki.sequence,
        }
    }
}
