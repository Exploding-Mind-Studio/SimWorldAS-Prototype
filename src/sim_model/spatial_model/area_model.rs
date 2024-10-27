// AREA Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

/*
    Design: Landscape is the core data model for spatial representation in SimWorld.
    It provides a core lattice of X by Y coordinates (in a Vector) with Height.
    
    Locations are sparse data sets that link back to the Landscape via X, Y coordinates,
    and contain various Landscape Cell properties. (Cell being the representation)

    Each Cell can be associated with multiple Areas/Zones [DIFFERENTIATION?]
    E.g. Climate Region, Biome, Soil composition, Urban/Rural Area
*/