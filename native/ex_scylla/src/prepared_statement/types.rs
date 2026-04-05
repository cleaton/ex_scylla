use rustler::{NifStruct, ResourceArc};
use scylla::{frame::response::result::PartitionKeyIndex, statement::prepared::PreparedStatement};

use crate::{
    session::types::*,
    utils::{to_elixir, ToElixir},
};

pub struct PreparedStatementResource(pub PreparedStatement);
impl std::panic::RefUnwindSafe for PreparedStatementResource {}

to_elixir!(
    PreparedStatement,
    ResourceArc<PreparedStatementResource>,
    |ps: PreparedStatement| { ResourceArc::new(PreparedStatementResource(ps)) }
);

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.PreparedMetadata"]
pub struct ScyllaPreparedMetadata {
    pub col_count: usize,
    pub pk_indexes: Vec<ScyllaPartitionKeyIndex>,
    pub col_specs: Vec<ScyllaColumnSpec>,
}

impl From<&PreparedStatement> for ScyllaPreparedMetadata {
    fn from(ps: &PreparedStatement) -> Self {
        let col_specs = ps.get_variable_col_specs();
        let pk_indexes = ps.get_variable_pk_indexes();

        ScyllaPreparedMetadata {
            col_count: col_specs.iter().count(),
            pk_indexes: pk_indexes.iter().map(|pki| (*pki).into()).collect(),
            col_specs: col_specs.iter().map(|cs| cs.clone().ex()).collect(),
        }
    }
}

#[derive(NifStruct, Debug)]
#[module = "ExScylla.Types.PartitionKeyIndex"]
pub struct ScyllaPartitionKeyIndex {
    pub index: u16,
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
