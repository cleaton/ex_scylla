use std::cell::Cell;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Mutex;

use rustler::NifTaggedEnum;
use scylla::SessionBuilder;
use scylla::transport::session::PoolSize;

use crate::session::types::ScyllaIpAddr;
use crate::utils::*;

pub struct SessionBuilderResource(pub Mutex<Cell<SessionBuilder>>);

impl Deref for SessionBuilderResource {
    type Target = Mutex<Cell<SessionBuilder>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(NifTaggedEnum)]
pub enum ScyllaPoolSize {
    PerHost(usize),
    PerShard(usize),
}

impl ToRust<PoolSize> for ScyllaPoolSize {
    fn r(self) -> PoolSize {
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

pub type ScyllaSocketAddr = (ScyllaIpAddr, u16);

impl ToRust<SocketAddr> for ScyllaSocketAddr {
    fn r(self) -> SocketAddr {
        let (sia, port) = self;
        SocketAddr::new(sia.into(), port)
    }
}
