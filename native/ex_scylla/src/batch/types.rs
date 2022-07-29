use crate::utils::*;
use rustler::NifUnitEnum;
use scylla::{batch::Batch, frame::request::batch::BatchType};

pub struct BatchResource(pub Batch);

clone_enum!(BatchType, ScyllaBatchType, {Logged, Unlogged, Counter});
