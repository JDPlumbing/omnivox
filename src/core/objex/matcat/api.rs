use crate::core::objex::matcat::{
    categories::{CategoryId, CATEGORY_MAP},
    variants::{VariantId, VARIANT_MAP},
    grades::{GradeId, GRADE_MAP},
    category_ranges::generate_props_from_category,
    properties::MatProps,
};

use serde::{Serialize, Deserialize};

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

    /// Serialize to 5-byte array (wire / storage format)
    pub fn to_bytes(&self) -> [u8; 5] {
        let [v1, v2] = self.variant.map(|v| v.0).unwrap_or(0).to_be_bytes();
        let [g1, g2] = self.grade.map(|g| g.0).unwrap_or(0).to_be_bytes();

        [self.category.0, v1, v2, g1, g2]
    }

    /// Deserialize from 5-byte array
    pub fn from_bytes(b: [u8; 5]) -> Self {
        let variant = u16::from_be_bytes([b[1], b[2]]);
        let grade = u16::from_be_bytes([b[3], b[4]]);
        Self::new(b[0], variant, grade)
    }

    /// Human-readable name with graceful fallbacks
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

    /// Generate material properties (category-level fallback for now)
    pub fn props(&self) -> Option<MatProps> {
        let mut rng = rand::rng();
        generate_props_from_category(self.category.0, &mut rng)
    }
}
