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

// lineage might imply hierarchy of ancestors.
// perhaps additonal component of "members" needed?
pub struct Lineage{
    line: Vec<SimEntity> // list of ancestors or constituent members from emergence
}

// Members is part of SimEntity Structure, 
// while Identity (Lineage) is separate in the canonical SimEntity model!
pub struct Members{
    member_list: Vec<SimEntity>
}

// refactor Origin out of Identity...
pub struct Origin{
    birth_location: Location,
    birth_time: TimeStamp, // timepoint of creation
    lineage: Lineage, // ancestors
    source: Source // process, cluster, emergence
}

// Identity
pub struct Identity {
    id: ID,
    name: Name, // initial iteration of complexity, just free form string label
    class: Class, // just a string label for now, no formal taxonomy
}

//====================================================

pub struct SimEntity{
    identity: Identity,
    origin: Origin    
}



