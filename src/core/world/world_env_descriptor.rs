use serde::{Deserialize, Serialize};
//use crate::core::world::WorldEnvironment;
//use crate::core::env::fields::Field;
//use crate::core::env::gravity::GravityField;
//use crate::core::env::medium::MediumField;
//use crate::core::env::atmosphere::AtmosphereField;
use crate::core::UvoxId;

//use std::sync::Arc;
use crate::core::env::medium::Medium;

/// How "up" is defined in this world.
/// You are explicitly choosing *not* flat space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpModel {
    /// Radial from world origin (Earth-style)
    Radial,

    /// Fixed axis (e.g. artificial habitats)
    Axial { axis: [f64; 3] },
}

/// Spatial assumptions of the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSpace {
    /// Radius of the solid surface (meters)
    /// This is the reference for altitude = 0
    pub surface_radius_m: f64,

    /// Defines what "up" means
    pub up_model: UpModel,
}

/// Gravity baseline for the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GravityKind {
    /// Gravity points toward the center
    Radial,

    /// Constant direction everywhere
    Uniform { direction: [f64; 3] },

    /// No gravity
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityModel {
    pub kind: GravityKind,

    /// Reference acceleration magnitude (m/s²)
    /// For Earth: ~9.80665
    pub strength: f64,
}

/// Default medium behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediumModel {
    /// What exists if nothing else overrides it
    pub default: Medium,
}

/// Optional atmospheric behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereModel {
    /// Density at surface (kg/m³)
    pub sea_level_density: f64,

    /// Exponential falloff scale height (meters)
    pub scale_height_m: f64,

    /// Optional hard cutoff
    pub max_height_m: Option<f64>,
}

/// Optional temperature model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureModel {
    /// Temperature at surface (K)
    pub surface_temp_k: f64,

    /// Optional linear lapse rate (K/m)
    pub lapse_rate_k_per_m: Option<f64>,
}

/// Optional pressure model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureModel {
    /// If true, pressure is derived from density + gravity
    pub derive_from_density: bool,
}

/// 🔑 This is the authoritative definition of a world's environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEnvDescriptor {
    // REQUIRED — invariants
    pub space: WorldSpace,
    pub gravity: GravityModel,
    pub medium: MediumModel,

    // OPTIONAL — flavor
    pub atmosphere: Option<AtmosphereModel>,
    pub temperature: Option<TemperatureModel>,
    pub pressure: Option<PressureModel>,
}

impl WorldSpace {
    pub fn altitude_m(&self, uvox: &UvoxId) -> f64 {
        uvox.radius_m() - self.surface_radius_m
    }
}
