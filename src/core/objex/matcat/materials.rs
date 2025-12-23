use once_cell::sync::Lazy;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use crate::core::objex::matcat::categories::{CategoryId, CATEGORY_MAP};
use crate::core::objex::matcat::variants::{VariantId, VARIANT_MAP};
use crate::core::objex::matcat::grades::{GradeId, GRADE_MAP};
use crate::core::objex::matcat::category_ranges::generate_props_from_category;
use crate::core::objex::matcat::properties::MatProps;

/// Material Categorization ID (5-byte logical identity)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatCatId {
    pub category: CategoryId,
    pub variant: Option<VariantId>,
    pub grade: Option<GradeId>,
}

impl MatCatId {
    pub fn new(category: u8, variant: u16, grade: u16) -> Self {
        Self {
            category: CategoryId(category),
            variant: if variant == 0 { None } else { Some(VariantId(variant)) },
            grade: if grade == 0 { None } else { Some(GradeId(grade)) },
        }
    }

    pub fn category_only(category: u8) -> Self {
        Self {
            category: CategoryId(category),
            variant: None,
            grade: None,
        }
    }

    /// Collapse identity into a deterministic seed
    pub fn seed(&self) -> u64 {
        let cat = self.category.0 as u64;
        let var = self.variant.map(|v| v.0 as u64).unwrap_or(0);
        let grd = self.grade.map(|g| g.0 as u64).unwrap_or(0);
        (cat << 32) | (var << 16) | grd
    }

    /// Human-readable name with graceful fallback
    pub fn name(&self) -> String {
        let cat = CATEGORY_MAP
            .get(&self.category)
            .copied()
            .unwrap_or("Unknown");

        let variant = self
            .variant
            .and_then(|v| VARIANT_MAP.get(&(self.category, v)).copied())
            .unwrap_or("Generic");

        let grade = match (self.variant, self.grade) {
            (Some(v), Some(g)) => GRADE_MAP
                .get(&(self.category, v, g))
                .copied()
                .unwrap_or("Standard"),
            _ => "Standard",
        };

        format!("{cat} - {variant} - {grade}")
    }


    /// Convenience constructors
    pub fn steel_lowcarbon() -> Self {
        Self::new(1, 1, 1)
    }
    pub fn metal_cu() -> Self {
        Self::new(1, 3, 1)
    }
    pub fn masonry_generic() -> Self {
        Self::new(9, 1, 0)
    }
    pub fn gas_air() -> Self {
        Self::new(13, 1, 0)
    }
    pub fn liquid_water() -> Self {
        Self::new(27, 1, 1)
    }
    pub fn plasma_stellar() -> Self {
        Self::new(26, 1, 1)
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "steel" => Some(Self::steel_lowcarbon()),
            "copper" => Some(Self::metal_cu()),
            "concrete" => Some(Self::masonry_generic()),
            "water" => Some(Self::liquid_water()),
            "air" => Some(Self::gas_air()),
            "plasma" => Some(Self::plasma_stellar()),
            _ => None,
        }
    }
}

/// Deterministic procedural fallback PRNG
fn lcg(mut seed: u64) -> impl FnMut() -> f32 {
    move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = (seed >> 32) as u32;
        bits as f32 / u32::MAX as f32
    }
}

/// Cached resolved material properties
static MATPROPS_CACHE: Lazy<Mutex<HashMap<MatCatId, MatProps>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Resolve material properties for a given MatCatId
pub fn props_for(id: &MatCatId) -> MatProps {
    // Cache hit
    if let Some(props) = MATPROPS_CACHE.lock().unwrap().get(id).copied() {
        return props;
    }

    let mut rng = StdRng::seed_from_u64(id.seed());

    // Special-case plasma (explicit physics override)
    let props = if id.category.0 == 26 {
        MatProps {
            density: 0.0002,
            elastic_modulus: 0.0,
            tensile_strength: 0.0,
            compressive_strength: 0.0,
            hardness: 0.0,
            fracture_toughness: 0.0,
            fatigue_resistance: 0.0,

            thermal_conductivity: 1.0e5,
            thermal_expansion: 0.0,
            melting_point: 6000.0,
            specific_heat: 1.0e4,

            corrosion_resistance: 1.0,
            solubility: 0.0,
            permeability: 1.0,
            flammability: 0.0,

            electrical_conductivity: 1.0,
            magnetic_permeability: 1.0,

            refractive_index: 1.0,
            transparency: 0.0,
            reflectivity: 0.1,
            absorption: 1.0,

            uv_resistance: 1.0,
        }
    } else if let Some(props) =
        generate_props_from_category(id.category.0, &mut rng)
    {
        props
    } else {
        // Absolute fallback (should almost never happen)
        let mut r = lcg(id.seed());
        MatProps {
            density: 500.0 + r() * 20000.0,
            elastic_modulus: r() * 4e11,
            tensile_strength: r() * 2000.0,
            compressive_strength: r() * 4000.0,
            hardness: r() * 10.0,
            fracture_toughness: r() * 50.0,
            fatigue_resistance: r(),

            thermal_conductivity: r() * 400.0,
            thermal_expansion: r() * 1e-4,
            melting_point: r() * 4000.0,
            specific_heat: 100.0 + r() * 2000.0,

            corrosion_resistance: r(),
            solubility: r(),
            permeability: r(),
            flammability: r(),

            electrical_conductivity: r(),
            magnetic_permeability: r() * 1000.0,

            refractive_index: 1.0 + r() * 1.5,
            transparency: r(),
            reflectivity: r(),
            absorption: r(),

            uv_resistance: r(),
        }
    };

    MATPROPS_CACHE.lock().unwrap().insert(*id, props);
    props
}
