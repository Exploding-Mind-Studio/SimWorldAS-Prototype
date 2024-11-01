// SIMULATION ENGINE Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)
// BD 2024-10-31 : filled in some initial data structures

// central config for all /sim_engine/ folder modules...
// - Sim Loop
// - Time Progress & Aggregation
// - Emergence Monitor & Facilitator
// - System Integration


// monitors SimWorld Entities for state, structure, lifecycle etc 
// (3 Complexity Dimensions: life cycle, cluster, level of detail)
// and looks if conditions are given to give rise to emergent features
// to be manifested as higher order SimEntities (spawned by the next EmergenceFacilitator)

pub struct SimRuntime{ // the simulation runtime

}
pub trait SimRunLoop{}
pub trait SimTimeTick{}
pub trait AdvanceTime{}
 pub struct StateChangeLog{}
 trait LogStateChang{}

 pub struct EventLog{}
 trait LogEvent{}



pub struct EmergenceMonitor{}


// instantiated new SimEntities of higher order of complexity, emergent from combinatory constellations of states/structures/events from lower level
pub struct EmergenceFacilitator{}