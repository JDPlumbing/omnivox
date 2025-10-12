pub mod core;
pub mod systems;
pub mod error;

pub use error::{ObjexError, Result};

pub use core::{
    Object,
    CompositeObject,
    types::{Objex, Shape, MaterialLink},
};

pub use systems::{
    mass,
    strength,
    thermal,
    degradation,
    mechanical,
    composite as systems_composite,
    electrical,
};