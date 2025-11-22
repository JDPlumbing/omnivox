use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::core::objex::matcat::category_ranges::generate_props_from_category;
use rand::{SeedableRng, rngs::StdRng};
use crate::core::categories;
use crate::core::variants;
use crate::core::grades;

/// 5-byte compact material identifier

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MatCatId {
    pub category: u8,
    pub variant: u16,
    pub grade: u16,
}


impl MatCatId {
    pub fn new(category: u8, variant: u16, grade: u16) -> Self {
        Self { category, variant, grade }
    }

    /// Collapse into a single 64-bit seed for procedural generation
    pub fn seed(&self) -> u64 {
        ((self.category as u64) << 32) | ((self.variant as u64) << 16) | (self.grade as u64)
    }
    
     /// Serialize to 5-byte array
    pub fn name(&self) -> String {
        let cat = categories::CATEGORY_MAP.get(&self.category).unwrap_or(&"Unknown");
        let variant = variants::VARIANT_MAP
            .get(&(self.category, self.variant))
            .unwrap_or(&"Generic");
        let grade = grades::GRADE_MAP
            .get(&(self.category, self.variant, self.grade))
            .unwrap_or(&"Standard");
        format!("{cat} - {variant} - {grade}")
    }

    pub fn props(&self) -> Option<crate::core::objex::matcat::materials::MatProps> {
        let mut rng = rand::rng();
        crate::core::objex::matcat::category_ranges::generate_props_from_category(self.category, &mut rng)
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "concrete" => Some(MatCatId::masonry_generic()),
            "steel" => Some(MatCatId::steel_lowcarbon()),
            "copper" => Some(MatCatId::metal_cu()),
            "water" => Some(MatCatId::liquid_water()),
            "air" => Some(MatCatId::gas_air()),
            "plasma" => Some(MatCatId::plasma_stellar()),

            _ => None,
        }
    }
}

/// Canonical material property set.
/// Every material is defined by these values, regardless of name.
#[derive(Debug, Clone, Copy)]
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
    pub specific_heat: f32,       // J/kg·K
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

    pub uv_resistance: f32,    // 0.0–1.0
    

}

/// Small deterministic PRNG for procedural props
fn lcg(mut seed: u64) -> impl FnMut() -> f32 {
    move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = (seed >> 32) as u32;
        (bits as f32) / (u32::MAX as f32) // → [0.0, 1.0]
    }
}
pub fn restitution_from_props(props: &MatProps) -> f64 {
    // Approximation: elasticity ratio → restitution coefficient
    // Metals ≈ 0.7–0.9, ceramics ≈ 0.1–0.3, polymers ≈ 0.4–0.6
    let e = props.elastic_modulus as f64;
    let toughness = props.fracture_toughness as f64;
    ((e / (e + toughness.max(1.0))) * 0.9).clamp(0.05, 0.95)
}

/// Procedurally derive material properties from MatCatId

static MATPROPS_CACHE: Lazy<Mutex<HashMap<MatCatId, MatProps>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn props_for(id: &MatCatId) -> MatProps {
    // First, try the cache
    {
        let cache = MATPROPS_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(id) {
            return *cached;
        }
    }
    let mut rng = StdRng::seed_from_u64(id.seed());
        if id.category == 26 {

        // Explicit stellar plasma properties
        let props = MatProps {
            // Mechanical (mostly irrelevant for plasma)
            density: 0.0002,             // kg/m³ (solar photosphere)
            elastic_modulus: 0.0,
            tensile_strength: 0.0,
            compressive_strength: 0.0,
            hardness: 0.0,
            fracture_toughness: 0.0,
            fatigue_resistance: 0.0,

            // Thermal
            thermal_conductivity: 1.0e5_f32,
            thermal_expansion: 0.0,
            melting_point: 6000.0_f32,
            specific_heat: 1.0e4_f32,


            // Chemical
            corrosion_resistance: 1.0,
            solubility: 0.0,
            permeability: 1.0,
            flammability: 0.0,

            // Electrical / Magnetic
            electrical_conductivity: 1.0,     // max normalized
            magnetic_permeability: 1.0,

            // Optical
            refractive_index: 1.0,
            transparency: 0.0,
            reflectivity: 0.1,
            absorption: 1.0,

            uv_resistance: 1.0,
        };

        // cache and return
        MATPROPS_CACHE.lock().unwrap().insert(*id, props);
        return props;
    }

    let props = if let Some(props) = generate_props_from_category(id.category, &mut rng) {
        props
    } else {
        let mut rng = lcg(id.seed());

        MatProps {
            density: 500.0 + rng() * 20000.0,
            elastic_modulus: rng() * 4e11,
            tensile_strength: rng() * 2000.0,
            compressive_strength: rng() * 4000.0,
            hardness: rng() * 10.0,
            fracture_toughness: rng() * 50.0,
            fatigue_resistance: rng(),
            thermal_conductivity: rng() * 400.0,
            thermal_expansion: rng() * 1e-4,
            melting_point: rng() * 4000.0,
            specific_heat: 100.0 + rng() * 2000.0,
            corrosion_resistance: rng(),
            solubility: rng(),
            permeability: rng(),
            flammability: rng(),
            electrical_conductivity: rng(),
            magnetic_permeability: rng() * 1000.0,
            refractive_index: 1.0 + rng() * 1.5,
            transparency: rng(),
            reflectivity: rng(),
            absorption: rng(),
            uv_resistance: rng(),


        }
    };

    // Store in cache
    MATPROPS_CACHE.lock().unwrap().insert(*id, props);
    props
}



use std::f32;

/// Distance metric between two materials (Euclidean in property space).
fn distance(a: &MatProps, b: &MatProps) -> f32 {
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
        (a.uv_resistance - b.uv_resistance).powi(2),
    ];

    diffs.iter().sum::<f32>().sqrt()
}

/// Find the closest material ID to some target properties.
pub fn find_closest_material(target: &MatProps, search_space: &[MatCatId]) -> Option<(MatCatId, MatProps)> {
    let mut best_id = None;
    let mut best_props = None;
    let mut best_dist = f32::MAX;

    for id in search_space {
        let props = props_for(id);
        let d = distance(&props, target);
        if d < best_dist {
            best_dist = d;
            best_id = Some(*id);
            best_props = Some(props);
        }
    }

    best_id.zip(best_props)
}

impl MatCatId {
    pub fn steel_lowcarbon() -> Self { Self::new(1, 1, 1) }
    pub fn metal_cu() -> Self { Self::new(1, 2, 1) }
    pub fn masonry_generic() -> Self { Self::new(2, 1, 1) }
    pub fn gas_air() -> Self { Self::new(3, 1, 1) }
    pub fn liquid_water() -> Self { Self::new(4, 1, 1) }
    pub fn plasma_stellar() -> Self {
        // Category 26 = Plasma
        Self::new(26, 1, 1)
    }

}


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
