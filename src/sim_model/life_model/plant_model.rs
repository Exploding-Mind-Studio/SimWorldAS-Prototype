// PLANT MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// SCOPE:
// - Simulated entities basics in common with all vegetation, flora....
//   - inherit organism_model features
//   - growth patterns
//   - environmental impact (cross-system metabolism)
//   - clustering (e.g. meadows, forests)

use crate::sim_model::{
    world_model::*,
    entity_model::*,
    life_model::organism_model::*,
};

pub struct SimPlant{
    id: Identity
}

pub enum PlantClass{ // intended use by SimPlant::Identity::Class
    Grass,
    Bush,
    Tree, 
    Flower,
    Fruit,
    Vegetable
}