// ORGANISM MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// SCOPE:
// - Simulated entities basics in common with all life....
//   - Life Cycle
//   - Health
//   - Metabolism

use crate::sim_model::world_model::*;
use crate::sim_model::entity_model::*;


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

// alternatively... (still PoC'ing)
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
