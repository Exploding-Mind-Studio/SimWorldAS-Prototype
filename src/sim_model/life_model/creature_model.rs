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

use crate::sim_model::{
    world_model::*,
    entity_model::*,
    life_model::organism_model::*,
};

pub struct Memory{

} 

// more complex health system for SimCreature...
pub struct HealthLevel{
    class: HealthClass,
    level: Level
}
pub enum HealthClass{
    BodyStructure,
    BodySurface, // skin
    VascularSystem,
    RespirationSystem,
    NervousSystem,
    DigestionSystem,
    ActuationSystem,
    SensorySystem,
    MemorySystem,
}
pub enum Capabilities{ // type and level
    Perception(Perception),
    Cognition(Cognition),
    Reflex(Reflex),
    Dexterity(Dexterity),
    Stamina(Stamina),
    Strength(Strength),
    Speed(Speed), 
    Skill(Skill)
}

// specialization of Capabilities...
pub enum Perception{
    Vision,
    Hearing,
    Taste,
    Smell,
    TouchPressure,
    TouchTemperature,
    ElectroMagnetic
}

pub enum Cognition{ // abiliy level (innate/learned/impaired)
    Recognition(Level),
    Memorize(Level),
    Recall(Level),
    FollowGoal(Level),
    Search(Level),
    Plan(Level),
    ProblemSolve(Level),
    Deduct(Level),
    Induct(Level)
}

pub enum Reflex{}
pub enum Dexterity{}

pub enum Strength{}

pub enum Stamina{}
pub enum Speed{}

pub enum Skill{}



pub struct SimCreature{
    id: Identity,
    location: Location,
    health: HealthLevel,
    lifecycle_stage: LifeCycle   
}