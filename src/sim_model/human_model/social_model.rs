// SOCIAL MODEL Rust module for SimWorldAS-Prototype
// BD 2024-11-01 : Initial Version 

// SCOPE:
//   - Human social behavior (complex model, hence own module)

use crate::sim_model::{
    entity_model::*,
    life_model::organism_model::*,
    life_model::creature_model::*,
    life_model::human_model::*
};


// Data Structures...

pub enum FamilyRelations{
    Grandparent,
    Parent,
    Child,
    Cousin,
    AuntUncle,
}
pub enum RelationshipType{
    Relative,
    Tribal,
    SharedFaith,
    Work,
    Community
}


pub struct SocialRelationship{
    who:        SimHuman,
    knows_whom: SimHuman,
    relationship_type: RelationshipType,
    relationship_cause: RelationshipCause
}