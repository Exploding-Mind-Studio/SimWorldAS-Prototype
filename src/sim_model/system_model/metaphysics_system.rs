// METAPHYSICS SYSTEM for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

/*
    MetaPhysics in SimWorld is an abtracted, stylized, but coherent system of logic,
    that defines the fundamental elements and forces in the simulated world, 
    that everything else depends on and evolved from.
    Partially as narrative, partially as fractal (self-similar) patterns, consistent 
    across multiple scale levels.
*/

use crate::sim_model::{
    world_model::*,
    system_model::*
};

pub struct Element{
    name: Name
}
