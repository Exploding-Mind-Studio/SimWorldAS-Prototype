// CREATURE MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// SCOPE:
// - Simulated entities basics in common with all fauna....
//   - inherit organism_model features
//   - growth patterns
//   - species & traits (appearance, behavior) (genes?)
//   - social clustering (packs, herds)
//   - movement, roaming
//   - reflexes
//   - limited cognitive: memory, primitive goal pursuit
//   - environmental impact (symbiotic dynamics)
//   - predator/prey/vulture roles


use crate::sim_model::entity_model::*;
use crate::sim_model::life_model::organism_model::*;

pub struct SimCreature{
    id: Identity,
    location: Location,
    health: HealthLevel,
    lifecycle_stage: LifeCycle   
}