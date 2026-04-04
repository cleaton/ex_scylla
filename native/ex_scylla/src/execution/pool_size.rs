use std::convert::TryInto;

use rustler::NifTaggedEnum;
use scylla::client::PoolSize;

#[derive(NifTaggedEnum)]
pub enum ScyllaPoolSize {
    PerHost(usize),
    PerShard(usize),
}

impl From<ScyllaPoolSize> for PoolSize {
    fn from(val: ScyllaPoolSize) -> Self {
        match val {
            ScyllaPoolSize::PerHost(v) => {
                PoolSize::PerHost(v.try_into().expect("invalid per-host pool size"))
            }
            ScyllaPoolSize::PerShard(v) => {
                PoolSize::PerShard(v.try_into().expect("invalid per-shard pool size"))
            }
        }
    }
}
