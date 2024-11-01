// WEATHER SYSTEM for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// PURPOSE:
// The logic that dynamically updates Atmosphere, Hydrosphere, Geosphere as
// spatial cellular automaty with gradient descent logic, to simulate WEATHER

use crate::sim_model::spatial_model::{
    atmosphere_model::*,
    hydrosphere_model::*,
    geosphere_model::*
};
