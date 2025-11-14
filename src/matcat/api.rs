use crate::matcat::{categories::CATEGORY_MAP, variants::VARIANT_MAP, grades::GRADE_MAP, category_ranges::generate_props_from_category};
use rand::thread_rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatCatId {
    pub category: u8,
    pub variant: u16,
    pub grade: u16,
}

impl MatCatId {
    pub fn new(category: u8, variant: u16, grade: u16) -> Self {
        Self { category, variant, grade }
    }

    /// Serialize to 5-byte array
    pub fn to_bytes(&self) -> [u8; 5] {
        let [v1, v2] = self.variant.to_be_bytes();
        let [g1, g2] = self.grade.to_be_bytes();
        [self.category, v1, v2, g1, g2]
    }

    /// Deserialize from 5 bytes
    pub fn from_bytes(b: [u8; 5]) -> Self {
        let variant = u16::from_be_bytes([b[1], b[2]]);
        let grade = u16::from_be_bytes([b[3], b[4]]);
        Self { category: b[0], variant, grade }
    }

    /// Get human-readable name with fallbacks
    pub fn name(&self) -> String {
        let cat = CATEGORY_MAP.get(&self.category).unwrap_or(&"Unknown");
        let variant = VARIANT_MAP.get(&(self.category, self.variant)).unwrap_or(&"Generic");
        let grade = GRADE_MAP.get(&(self.category, self.variant, self.grade)).unwrap_or(&"Standard");
        format!("{cat} - {variant} - {grade}")
    }

    /// Generate material properties (category fallback logic)
    pub fn props(&self) -> Option<crate::matcat::materials::MatProps> {
        let mut rng = rand::rng();
        // for now, we just go by category until variant-level ranges exist
        generate_props_from_category(self.category, &mut rng)
    }
}
/*
#[get("/api/matcat/<category>/<variant>/<grade>")]
pub fn get_material(category: u8, variant: u16, grade: u16) -> Json<MaterialResponse> {
    use crate::matcat::materials::{MatCatId, props_for};

    let id = MatCatId::new(category, variant, grade);
    let props = props_for(&id);

    Json(MaterialResponse {
        id,
        name: id.name(),
        props,
    })
}
*/