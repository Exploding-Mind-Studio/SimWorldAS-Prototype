// SimEntity Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// Define shared properties and patterns of all things SimWorld
// - generic template, not ever instantiated by itself

use super::world_model::*;

// Common Components...

// Location 
pub struct Location{
    x_cor: Coordinate,
    y_cor: Coordinate,
    height: Height
}

// Identity
pub struct Identity {
    id: ID,
    name: Name,
    class: Class,
    origin: Origin, // location of creation (can be process)
    inception: TimeStamp // timepoint of creation
}

pub struct SimEntity{
    
}



