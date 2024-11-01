// PLACE Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)


/*
    Place is a socio-cultural concept, 
    which needs to be mapped back to the physical space/world.

    Think of cities, landmarks, landscape features humans and creatures might orient on.
*/

use crate::sim_model::{
    world_model::*,
    entity_model::*,
    object_model::*,
    resource_model::*,
};

pub struct SimPlace{
    label: Label,
    category: Category,
    location: Location,
    people: Vec<SimHuman>, // who is there right now
    objects: Vec<SimObject>, // what objects are there
    objects: Vec<SimResource> // resources here
}