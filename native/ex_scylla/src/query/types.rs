use crate::utils::*;
use rustler::ResourceArc;
use scylla::statement::unprepared::Statement as Query;

pub struct QueryResource(pub Query);
impl std::panic::RefUnwindSafe for QueryResource {}
impl rustler::Resource for QueryResource {}

to_elixir!(Query, ResourceArc<QueryResource>, |q: Query| {
    ResourceArc::new(QueryResource(q))
});
