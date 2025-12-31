use serde::{Serialize, Deserialize};

/// Canonical material property set.
/// Physics consumes this. Identity does not.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MatProps {
    // --- Mechanical ---
    pub density: f32,              // kg/m³
    pub elastic_modulus: f32,      // GPa
    pub tensile_strength: f32,     // MPa
    pub compressive_strength: f32, // MPa
    pub hardness: f32,             // Mohs-like 0–10
    pub fracture_toughness: f32,   // MPa·m^0.5
    pub fatigue_resistance: f32,   // 0.0–1.0

    // --- Thermal ---
    pub thermal_conductivity: f32, // W/m·K
    pub thermal_expansion: f32,    // 1/K
    pub melting_point: f32,        // °C
    pub specific_heat: f32,        // J/kg·K

    // --- Chemical ---
    pub corrosion_resistance: f32, // 0.0–1.0
    pub solubility: f32,           // 0.0–1.0
    pub permeability: f32,         // 0.0–1.0
    pub flammability: f32,         // 0.0–1.0

    // --- Electrical / Magnetic ---
    pub electrical_conductivity: f32, // 0.0–1.0
    pub magnetic_permeability: f32,   // relative μ

    // --- Optical ---
    pub refractive_index: f32,  // e.g. 1.0–2.5
    pub transparency: f32,      // 0.0–1.0
    pub reflectivity: f32,      // 0.0–1.0
    pub absorption: f32,        // 0.0–1.0

    // --- Environmental ---
    pub uv_resistance: f32,    // 0.0–1.0
}

/// Approximate coefficient of restitution derived from elastic behavior.
pub fn restitution_from_props(props: &MatProps) -> f64 {
    let e = props.elastic_modulus as f64;
    let toughness = props.fracture_toughness as f64;
    ((e / (e + toughness.max(1.0))) * 0.9).clamp(0.05, 0.95)
}

/// Distance metric between two materials in property space.
/// Used for similarity queries and approximation.
pub fn distance(a: &MatProps, b: &MatProps) -> f32 {
    let diffs = [
        (a.density - b.density).powi(2),
        (a.elastic_modulus - b.elastic_modulus).powi(2),
        (a.tensile_strength - b.tensile_strength).powi(2),
        (a.compressive_strength - b.compressive_strength).powi(2),
        (a.hardness - b.hardness).powi(2),
        (a.fracture_toughness - b.fracture_toughness).powi(2),
        (a.fatigue_resistance - b.fatigue_resistance).powi(2),

        (a.thermal_conductivity - b.thermal_conductivity).powi(2),
        (a.thermal_expansion - b.thermal_expansion).powi(2),
        (a.melting_point - b.melting_point).powi(2),
        (a.specific_heat - b.specific_heat).powi(2),

        (a.corrosion_resistance - b.corrosion_resistance).powi(2),
        (a.solubility - b.solubility).powi(2),
        (a.permeability - b.permeability).powi(2),
        (a.flammability - b.flammability).powi(2),

        (a.electrical_conductivity - b.electrical_conductivity).powi(2),
        (a.magnetic_permeability - b.magnetic_permeability).powi(2),

        (a.refractive_index - b.refractive_index).powi(2),
        (a.transparency - b.transparency).powi(2),
        (a.reflectivity - b.reflectivity).powi(2),
        (a.absorption - b.absorption).powi(2),

        (a.uv_resistance - b.uv_resistance).powi(2),
    ];

    diffs.iter().sum::<f32>().sqrt()
}

/// Reasonable fallback material used when nothing else is known.
pub fn default_props() -> MatProps {
    MatProps {
        density: 1000.0,
        elastic_modulus: 1e9,
        tensile_strength: 10.0,
        compressive_strength: 20.0,
        hardness: 5.0,
        fracture_toughness: 5.0,
        fatigue_resistance: 0.5,

        thermal_conductivity: 0.5,
        thermal_expansion: 1e-5,
        melting_point: 150.0,
        specific_heat: 500.0,

        corrosion_resistance: 0.5,
        solubility: 0.5,
        permeability: 0.5,
        flammability: 0.5,

        electrical_conductivity: 0.5,
        magnetic_permeability: 1.0,

        refractive_index: 1.5,
        transparency: 0.5,
        reflectivity: 0.3,
        absorption: 0.2,

        uv_resistance: 0.5,
    }
}
