// World_Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

use crate::sim_model::*;

// World Properties...

pub type Coordinate = u16;
pub type Height = u16;
pub type Depth = u16;
pub type Temperature = f32;
pub type ID = u32;
pub type Label = String;
pub type Name = String;
pub type Class = String;
// obsolete here ... pub type Lineage = String;
pub type Origin = String;
pub type TimeStamp = u64;
pub type Level = f32;
pub type Source = String;

pub struct SimWorld{
    id: ID,
    label: Label
}
