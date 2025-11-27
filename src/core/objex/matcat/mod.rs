//! # matcat
//!
//! A compact material catalog system.
//!
//! Materials are identified by a 5-byte code (`MatCatId`) and deterministically
//! expanded into full property sets (`MatProps`) using procedural generation.
//!
//! Public API is kept flat: just `use matcat::...`.
use crate::core::objex::core::MaterialName;
pub mod categories;
pub mod variants;
pub mod materials;
pub mod category_ranges;
pub mod api;
pub mod grades;

pub use materials::{
    MatCatId,
    MatProps,
    props_for,
    find_closest_material,
};

pub use category_ranges::*;
/*
impl MatCatId {
    pub fn from_name(name: &MaterialName) -> Option<Self> {
        match name {
            MaterialName::Steel => Some(MatCatId::steel_lowcarbon()),
            MaterialName::Copper => Some(MatCatId::metal_cu()),
            MaterialName::Concrete => Some(MatCatId::masonry_generic()),
            MaterialName::Air => Some(MatCatId::gas_air()),
            MaterialName::Water => Some(MatCatId::liquid_water()),
            MaterialName::Plasma => Some(MatCatId::plasma_stellar()),

            _ => None,
        }
    }
}
    */
impl MatCatId {
    pub fn from_name(name: &MaterialName) -> Option<Self> {
        match name {
            MaterialName::Steel        => Some(MatCatId::steel_lowcarbon()),
            MaterialName::Copper       => Some(MatCatId::metal_cu()),
            MaterialName::Aluminum     => Some(MatCatId::metal_cu()), // or custom
            MaterialName::Concrete     => Some(MatCatId::masonry_generic()),
            MaterialName::Air          => Some(MatCatId::gas_air()),
            MaterialName::Water        => Some(MatCatId::liquid_water()),
            MaterialName::Soil         => Some(MatCatId::masonry_generic()),
            MaterialName::Vacuum       => Some(MatCatId::gas_air()),   // or 0
            MaterialName::Plasma       => Some(MatCatId::plasma_stellar()),
            MaterialName::Custom(_)    => None,
            _                          => None,
        }
    }
}
