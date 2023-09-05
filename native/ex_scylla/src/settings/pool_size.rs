use std::convert::TryInto;

use rustler::NifTaggedEnum;
use scylla::transport::session::PoolSize;

#[derive(NifTaggedEnum)]
pub enum ScyllaPoolSize {
    PerHost(usize),
    PerShard(usize),
}

impl Into<PoolSize> for ScyllaPoolSize {
    fn into(self) -> PoolSize {
        match self {
            Self::PerHost(v) => {
                PoolSize::PerHost(v.try_into().expect("invalid per-host pool size"))
            }
            Self::PerShard(v) => {
                PoolSize::PerShard(v.try_into().expect("invalid per-shard pool size"))
            }
        }
    }
}
