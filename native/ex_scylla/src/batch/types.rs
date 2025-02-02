use std::panic::RefUnwindSafe;
use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::{batch::Batch, batch::BatchType};

pub struct BatchResource(pub Batch);
impl RefUnwindSafe for BatchResource {}

clone_enum!(BatchType, ScyllaBatchType, {Logged, Unlogged, Counter});
