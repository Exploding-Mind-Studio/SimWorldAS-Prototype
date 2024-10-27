// LOCATION Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

use crate::sim_model::entity_model::*;

// Landscape
pub struct Landscape{
    cell_type: LandscapeType,
    cell: Vec<LandscapeCell> 
}

pub struct LandscapeCell{
    location: Location,
    properties: Vec<CellProperty>
}

pub enum LandscapeType{
    Earth(Height),
    Water(Depth),
    Atmosphere
}

pub struct CellProperty{
    temperature: Temperature
    density: u8

}