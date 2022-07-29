use rustler::ResourceArc;
use scylla::query::Query;

use crate::utils::{to_elixir, ToElixir};
pub struct QueryResource(pub Query);

to_elixir!(Query, ResourceArc<QueryResource>, |q: Query| {
    ResourceArc::new(QueryResource(q))
});
