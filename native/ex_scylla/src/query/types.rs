use rustler::ResourceArc;
use scylla::query::Query;
use std::panic::RefUnwindSafe;

use crate::utils::{to_elixir, ToElixir};
pub struct QueryResource(pub Query);
impl RefUnwindSafe for QueryResource {}

to_elixir!(Query, ResourceArc<QueryResource>, |q: Query| {
    ResourceArc::new(QueryResource(q))
});
