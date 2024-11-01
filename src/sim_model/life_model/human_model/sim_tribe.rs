// HUMAN MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)
// BD 2024-10-31 : Added more code and data structures, more derived from upstream sim entities
//                  implemented rudimentary properties, identity, health, etc.

// SCOPE:
// - Simulated entities basics in common with all Humans, Persons....
//   - inherit organism & creature model features
//   - growing up patterns
//   - ethnicities: appearance, predispositions, inherant abilities (genetic)
//   - social clustering (packs, herds)
//   - movement, roaming
//   - emotions
//   - cognitive: goal planning, sybolic communication, use of tools
//   - complex social relations & behavior

use crate::sim_model::entity_model::*;
use crate::sim_model::life_model::organism_model::*;
use crate::sim_model::life_model::creature_model::*;

/* 
pub enum Cognition{
    RecognizeEvent,
    RecognizeObject,
    RecognizeCreature,
    RecognizePerson,
    RecognizeLocation,
    FindLocation,
    Memorize,
    Recall,
    Search,
    Plan,
    FollowGoal,
    ProblemSolve
    
}
*/
    
pub struct SimHuman{
    id: Identity,
    location: Location,
    health: HealthLevel,
    lifecycle_stage: LifeCycle,
    capabilities: Vec<Capabilities>,
}

// TODO: implement the 3 dimensional complexity scale (see paper notes from 2024-10-30)
// D1 = Life Cycle (Entities get more complex by growing to maturity after creation)
// D2 = Clustering (aggregations of active entities make things more complex, as they interact)
// D3 = Leven of Detail complexity increasing (by filling in more features, logic -> combinatorial explosion)

