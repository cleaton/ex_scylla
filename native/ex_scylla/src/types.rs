use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::statement::Consistency;
use scylla::statement::SerialConsistency;
use scylla_cql::frame::Compression;

clone_enum!(Consistency, ScyllaConsistency, {
    Any,
    One,
    Two,
    Three,
    Quorum,
    All,
    LocalQuorum,
    EachQuorum,
    Serial,
    LocalSerial,
    LocalOne
});

clone_enum!(Compression, ScyllaTransportCompression, {
    Lz4,
    Snappy
});

clone_enum!(SerialConsistency, ScyllaSerialConsistency, {
    Serial,
    LocalSerial
});
