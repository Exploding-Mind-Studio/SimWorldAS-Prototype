// COMMON Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// COMMON features for SimGame
// - Initialize World Model (generate / load model)
// - Game Menu (Start/Continue, Load/Save, Settings, Player Profile)
// - Game Mode (Entertainment/Scientific Simulation)
// - Game State (dashboard/onground observer/interferer/impersonator)
// - Game Flow 

use crate::sim_model::world_model::*;

pub struct SimGame{} // Game Instance
pub trait StartGame{}
pub trait PauseGame{}
pub trait SaveGame{}
pub trait LoadGame{}
pub trait CreatePlayerProfile{}
pub trait LoadPlayerProfile{}


// Instantiate core World environment

pub fn init_world() -> world_model {

}

// Generate dynamic content

// or Load saved World Model
pub fn load_world(filename: String) -> world_model {

}

