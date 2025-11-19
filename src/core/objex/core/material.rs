
use crate::core::objex::matcat::{MatCatId, MatProps, props_for};
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
