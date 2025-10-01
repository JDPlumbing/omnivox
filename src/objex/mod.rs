pub mod core;
pub mod systems;
pub mod persist;
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

pub use persist::{insert_objex, fetch_objex};

pub mod builder;
pub mod defaults;

pub use builder::ObjexBuilder;
