use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::core::objex::matcat::categories::CategoryId;
use crate::core::objex::matcat::variants::VariantId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GradeId(pub u16);
/// Grade tiers for each (category, variant) combination.
/// Represented by the last 2 bytes of MatCatId.
pub static GRADE_MAP: Lazy<HashMap<(CategoryId, VariantId, GradeId), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Metals - Steel
    m.insert((CategoryId(1), VariantId(1), GradeId(1)), "A36 Structural");
    m.insert((CategoryId(1), VariantId(1), GradeId(2)), "304 Stainless");
    m.insert((CategoryId(1), VariantId(1), GradeId(3)), "4140 Alloy");
    m.insert((CategoryId(1), VariantId(1), GradeId(4)), "Tool Steel");

    // Metals - Aluminum
    m.insert((CategoryId(1), VariantId(2), GradeId(1)), "6061-T6");
    m.insert((CategoryId(1), VariantId(2), GradeId(2)), "7075-T6");
    m.insert((CategoryId(1), VariantId(2), GradeId(3)), "1100 Pure");

    // Metals - Copper
    m.insert((CategoryId(1), VariantId(3), GradeId(1)), "C110 Electrolytic Tough Pitch");
    m.insert((CategoryId(1), VariantId(3), GradeId(2)), "C122 Phosphor Deoxidized");

    // Concrete
    m.insert((CategoryId(9), VariantId(1), GradeId(1)), "M20");
    m.insert((CategoryId(9), VariantId(1), GradeId(2)), "M25");
    m.insert((CategoryId(9), VariantId(1), GradeId(3)), "M40");

    // Wood
    m.insert((CategoryId(3), VariantId(1), GradeId(1)), "Construction Grade");
    m.insert((CategoryId(3), VariantId(1), GradeId(2)), "Select Structural");
    m.insert((CategoryId(3), VariantId(1), GradeId(3)), "Cabinet Grade");

    // Plastics - PVC
    m.insert((CategoryId(2), VariantId(1), GradeId(1)), "Schedule 40");
    m.insert((CategoryId(2), VariantId(1), GradeId(2)), "Schedule 80");

    // Composites
    m.insert((CategoryId(4), VariantId(1), GradeId(1)), "Standard Modulus");
    m.insert((CategoryId(4), VariantId(1), GradeId(2)), "High Modulus");
    m.insert((CategoryId(4), VariantId(1), GradeId(3)), "Ultra-High Modulus");

    // Glass
    m.insert((CategoryId(5), VariantId(1), GradeId(1)), "Annealed");
    m.insert((CategoryId(5), VariantId(1), GradeId(2)), "Tempered");
    m.insert((CategoryId(5), VariantId(1), GradeId(3)), "Laminated");

    // Ceramics
    m.insert((CategoryId(6), VariantId(1), GradeId(1)), "Low-Fire");
    m.insert((CategoryId(6), VariantId(1), GradeId(2)), "Mid-Fire");
    m.insert((CategoryId(6), VariantId(1), GradeId(3)), "High-Fire");

    // Rubbers
    m.insert((CategoryId(8), VariantId(1), GradeId(1)), "Soft");
    m.insert((CategoryId(8), VariantId(1), GradeId(2)), "Medium");
    m.insert((CategoryId(8), VariantId(1), GradeId(3)), "Hard");

    // Plasma
    m.insert((CategoryId(26), VariantId(1), GradeId(1)), "Default Stellar");

    // Water
    m.insert((CategoryId(27), VariantId(1), GradeId(1)), "Freshwater Standard");
    m.insert((CategoryId(27), VariantId(2), GradeId(1)), "Saltwater Standard");

    m
});


impl GradeId {
    pub fn name(self, category: CategoryId, variant: VariantId) -> &'static str {
        GRADE_MAP
            .get(&(category, variant, self))
            .copied()
            .unwrap_or("Generic")
    }
    pub fn exists(self, category: CategoryId, variant: VariantId) -> bool {
        GRADE_MAP.contains_key(&(category, variant, self))
    }
}

