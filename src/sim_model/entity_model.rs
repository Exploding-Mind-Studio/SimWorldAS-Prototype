// SimEntity Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// Define shared properties and patterns of all things SimWorld
// - generic template, not ever instantiated by itself


pub type Coordinate = u16;
pub type Height = u16;
pub type Depth = u16;
pub type Temperature = f32;
pub type ID = u32;
pub type Label = String;
pub type Name = String;


// Location 
pub struct Location{
    x_cor: Coordinate,
    y_cor: Coordinate,
    height: Height
}



