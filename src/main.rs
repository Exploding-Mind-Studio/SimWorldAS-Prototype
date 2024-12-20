// Main Program for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// including modules...
pub mod sim_engine;
pub mod sim_model;
pub mod sim_game;
pub mod config;

fn main() {
    let greeting = "
        Exploding Mind Studio presents... SimWorldAS Prototype
            implemented in pure Rust 
            
        What you will see in this prototype implementation:
        - SimWorld model: Spatial, Atmosphere, Objects, Organisms, Plants, Creatures, People
        - SimEngine executive: Time, Dynamics Activation
        - SimGame: User Experience

        Current Stage of this Build:
        -- mostly focused on establishing data structures for Models, Systems
        
        (c) 10/2024
    ";
    
    println!("{}\n", greeting);
}
