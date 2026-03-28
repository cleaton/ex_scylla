use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::statement::batch::{Batch, BatchType};

pub struct BatchResource(pub Batch);

clone_enum!(BatchType, ScyllaBatchType, {Logged, Unlogged, Counter});
