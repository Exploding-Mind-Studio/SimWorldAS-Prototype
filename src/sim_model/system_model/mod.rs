// SYSTEM MODEL Rust module for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// central config for all /system_model/ folder modules...
// - Metaphysics & Alchemy (Laws of Nature)
// - day/night cycle (regional light & temperature distribution)
// - regional seasons
// - solar radiation as heat input to Atmosphere, and Landscape
// - weather system (heat, air pressure, moisture, winds, precipitation, evaporation, fog, clouds)
// - water cycle
// - universe/sky (observable star/planet constellations)

pub mod metaphysics_system;
pub mod alchemy_system;
pub mod water_system;
pub mod weather_system;
pub mod sun_system;
pub mod moon_system;
pub mod sky_system;
