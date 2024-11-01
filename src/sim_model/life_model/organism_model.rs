// ORGANISM MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// SCOPE:
// - Simulated entities basics in common with all life....
//   - Life Cycle
//   - Health
//   - Metabolism

use crate::sim_model::{
    world_model::*,
    entity_model::*
};


// Components common to all Organisms and emergent life entities...

pub enum LifeCycle{ // common stages can be customized for each entity class via label  
    New(Label),
    Developing(Label),
    Maturing(Label),
    GrownUp(Label),
    Aging(Label),
    Dead(Label)
}

// should Health be implemented via Rust Generic Types?
// as a container for actual Health types(depending on entity class)
// Bevy might be more favorable to data structure types vs. Rust generic types?
pub struct Health{ // very generic, data content makes it specific
    class: Class, // type of health level
    level: Level
}

