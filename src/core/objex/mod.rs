pub mod core;
pub mod systems;
pub mod error;
pub mod templates;
pub mod geospec;
pub mod matcat;

pub use error::{ObjexError, Result};

pub use core::{
    CompositeObject,
    Objex,
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
/*(* --- IGNORE ---)
pub use templates::{
    Sun,
    Moon,
};
*/
pub use geospec::*;
pub use matcat::*;