//! # matcat
//!
//! A compact material catalog system.
//!
//! Materials are identified by a 5-byte code (`MatCatId`) and deterministically
//! expanded into full property sets (`MatProps`) using procedural generation.
//!
//! Public API is kept flat: just `use matcat::...`.
use crate::objex::core::types::MaterialName;
pub mod categories;
pub mod variants;
pub mod materials;
pub mod category_ranges;
pub mod api;
pub use api::*;
pub mod grades;

pub use materials::{
    MatCatId,
    MatProps,
    props_for,
    find_closest_material,
};

pub use category_ranges::*;

impl MatCatId {
    pub fn from_name(name: &MaterialName) -> Option<Self> {
        match name {
            MaterialName::Steel => Some(MatCatId::steel_lowcarbon()),
            MaterialName::Copper => Some(MatCatId::metal_cu()),
            MaterialName::Concrete => Some(MatCatId::masonry_generic()),
            MaterialName::Air => Some(MatCatId::gas_air()),
            MaterialName::Water => Some(MatCatId::liquid_water()),
            _ => None,
        }
    }
}
