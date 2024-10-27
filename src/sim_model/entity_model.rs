// SimEntity Model for SimWorldAS-Prototype
// BD 2024-10-27 : Initial Version (adapted from SimWorld24-Rust)

// Define shared properties and patterns of all things SimWorld
// - generic template, not ever instantiated by itself


type Coordinate = u16;
type Height = u16;
type Depth = u16;
type Temperature = f32;

// Location 
struct Location{
    x_cor: Coordinate,
    y_cor: Coordinate,
    height: Height
}


// Landscape
struct Landscape{
    cell_type: LandscapeType,
    cell: Vec<LandscapeCell> 
}

struct LandscapeCell{
    x_cor: Coordinate,
    y_cor: Coordinate,
    properties: Vec<CellProperty>
}

enum LandscapeType{
    Earth(Height),
    Water(Depth),
    Atmosphere
}

struct CellProperty{
    temperature: Temperature
}
