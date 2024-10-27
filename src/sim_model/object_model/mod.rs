// SimObject Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// Define types & properties of non-living things in SimWorld
// - derived from SimEntity

// The difference between entity_model and object_model:
// - SimEntity is a generic entity with reusable properties & patterns for all simulated aspects
// - SimObject sets the stage for any non-living, non-system things/structures, like houses, tools, rocks etc.
// That is why object_model gets its own module folder, there will be several derivates moduels for each object type.
// *("OBJECT" NOT MEANT IN THE SENSE OF OOP!)
