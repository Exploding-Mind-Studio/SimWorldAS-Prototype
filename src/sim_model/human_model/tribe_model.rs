// TRIBE MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-31 : Initial Version (adapted from SimWorld24-Rust)

// SCOPE:
//   - Human social cluster, emergent from individual SimHumans

use crate::sim_model::{
    entity_model::*,
    life_model::organism_model::*,
    life_model::creature_model::*,
    life_model::human_model::*
};

pub struct SimTribe{
    members: Vec<SimHuman>
}