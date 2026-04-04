use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::statement::batch::{Batch, BatchType};

pub struct BatchResource(pub Batch);
impl std::panic::RefUnwindSafe for BatchResource {}

clone_enum!(BatchType, ScyllaBatchType, {Logged, Unlogged, Counter});
