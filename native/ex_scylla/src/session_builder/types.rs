use scylla::SessionBuilder;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Mutex;

use crate::session::types::ScyllaIpAddr;
use crate::utils::*;
use std::cell::Cell;

pub struct SessionBuilderResource(pub Mutex<Cell<SessionBuilder>>);

impl Deref for SessionBuilderResource {
    type Target = Mutex<Cell<SessionBuilder>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type ScyllaSocketAddr = (ScyllaIpAddr, u16);

impl ToRust<SocketAddr> for ScyllaSocketAddr {
    fn r(self) -> SocketAddr {
        let (sia, port) = self;
        SocketAddr::new(sia.into(), port)
    }
}
