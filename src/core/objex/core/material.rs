
use crate::core::objex::matcat::{MatCatId,
                                CategoryId,
                                VariantId,
                                GradeId,
};
use crate::core::objex::matcat::properties::{MatProps};
use crate::core::objex::matcat::materials::props_for;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaterialKind {
    Metal,
    Ceramic,
    Polymer,
    Organic,
    Masonry,
    Glass,
    Liquid,
    Gas,
    Composite,
    Plasma,
    Geologic,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MaterialName {
    Steel,
    Copper,
    Aluminum,
    Concrete,
    Wood,
    Plastic,
    Rubber,
    Glass,
    Air,
    Water,
    Plasma,
    Soil,
    Vacuum,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialLink {
    pub name: MaterialName,  // optional: human readable
    pub kind: MaterialKind,  // broad classifier for systems
    pub matcat_id: MatCatId, // the canonical physics identity
}

impl MaterialLink {
    pub fn props(&self) -> MatProps {
        props_for(&self.matcat_id)
    }
    
}
impl MaterialLink {
    pub fn new(name: MaterialName) -> Self {
        let matcat_id = MatCatId::from_name(&name)
            .unwrap_or_else(|| MatCatId { 
                category: CategoryId(0), 
                variant: Some(VariantId(0)), 
                grade: Some(GradeId(0)    
            ) });

        // Infer kind automatically
        let kind = match name {
            MaterialName::Steel => MaterialKind::Metal,
            MaterialName::Copper => MaterialKind::Metal,
            MaterialName::Aluminum => MaterialKind::Metal,
            MaterialName::Concrete => MaterialKind::Masonry,
            MaterialName::Wood => MaterialKind::Organic,
            MaterialName::Plastic => MaterialKind::Polymer,
            MaterialName::Glass => MaterialKind::Glass,
            MaterialName::Air => MaterialKind::Gas,
            MaterialName::Water => MaterialKind::Liquid,
            MaterialName::Soil => MaterialKind::Geologic,
            MaterialName::Vacuum => MaterialKind::Gas,
            MaterialName::Rubber => MaterialKind::Polymer,
            MaterialName::Plasma => MaterialKind::Plasma,
            MaterialName::Custom(_) => MaterialKind::Other,
        };

        Self { name, kind, matcat_id }
    }

    pub fn vacuum() -> Self {
        Self::new(MaterialName::Vacuum)
    }
}

impl MaterialLink {
    pub fn stellar_plasma() -> Self {
        MaterialLink::new(MaterialName::Plasma)
    }
}
