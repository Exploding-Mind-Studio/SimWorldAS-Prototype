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


use crate::sim_model::{
    world_model::*,
    entity_model::*,
    life_model::organism_model::*,
    life_model::creature_model::*
};


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

pub enum Emotion{
    Calm, // relaxed
    Surprise,
    Disgust,
    Fear,
    Anger,  // contempt, envy
    Sadness, 
    Joy, // Happy
    Shame,
    Grief, // mourning loss
    // biblical...
    Greed,
    Envy
}

pub struct SimHuman{
    id: Identity,
    current_location: Location,
    current_health: HealthLevel,
    lifecycle_stage: LifeCycle,
    capabilities: Vec<Capabilities>,
    emotional_state: Vec<Emotion>
}

// TODO: implement the 3 dimensional complexity scale (see paper notes from 2024-10-30)
// D1 = Life Cycle (Entities get more complex by growing to maturity after creation)
// D2 = Clustering (aggregations of active entities make things more complex, as they interact)
// D3 = Leven of Detail complexity increasing (by filling in more features, logic -> combinatorial explosion)

// Challenge: how to manifest the Emergence transition in data structures?
// Via Lineage? Who points to whom?
// New clustered/life cyle advanced stage with new capabilities / more detailed LoD?
// Does Life Cycle stage advance really change anything in the Entity structure by itself?
// new, emerged entity, should point back to Lineage, and contain members, if it is a cluster
// like so...
pub struct simTribe{
    members: Vec<SimHuman>
}