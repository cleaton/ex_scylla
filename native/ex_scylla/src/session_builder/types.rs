use scylla::client::session_builder::SessionBuilder;
use std::cell::Cell;
use std::sync::Mutex;

pub struct SessionBuilderResource(pub Mutex<Cell<SessionBuilder>>);
impl std::panic::RefUnwindSafe for SessionBuilderResource {}
