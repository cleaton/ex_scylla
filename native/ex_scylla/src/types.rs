use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::batch::SerialConsistency;
use scylla::frame::types::Consistency;
use scylla::transport::Compression;

clone_enum!(Consistency, ScyllaConsistency, {
    Any,
    One,
    Two,
    Three,
    Quorum,
    All,
    LocalQuorum,
    EachQuorum,
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
