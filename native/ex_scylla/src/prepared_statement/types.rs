use rustler::{NifStruct, ResourceArc};
use scylla::{
    frame::response::result::{PartitionKeyIndex, PreparedMetadata},
    statement::prepared_statement::PreparedStatement,
};

use crate::{
    session::types::*,
    utils::{to_elixir, ToElixir},
};

pub struct PreparedStatementResource(pub PreparedStatement);

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

impl From<&PreparedMetadata> for ScyllaPreparedMetadata {
    fn from(pm: &PreparedMetadata) -> Self {
        ScyllaPreparedMetadata {
            col_count: pm.col_count,
            pk_indexes: pm
                .pk_indexes
                .to_owned()
                .into_iter()
                .map(|pki| pki.into())
                .collect(),
            col_specs: pm
                .col_specs
                .to_owned()
                .into_iter()
                .map(|cs| cs.ex())
                .collect(),
        }
    }
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
