use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::core::{WorldId, UvoxId};
use crate::supabasic::properties::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnchorDto {
    pub world_id: WorldId,
    pub uvox: UvoxId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyInput {
    pub address_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
    pub name: Option<String>,
    pub anchor: AnchorDto,

    pub property_type: Option<PropertyType>,
    pub square_feet: Option<i64>,
    pub sqft_under_air: Option<i64>,
    pub bedrooms: Option<i64>,
    pub bathrooms: Option<i64>,
    pub num_units: Option<i64>,
    pub stories: Option<i64>,
    pub num_buildings: Option<i64>,
    pub ceiling_height_ft: Option<f64>,
    pub foundation_type: Option<FoundationType>,
    pub exterior_type: Option<ExteriorType>,
    pub roof_type: Option<RoofType>,
    pub roof_material: Option<RoofMaterial>,
    pub plumbing: Option<PlumbingType>,
    pub electrical: Option<ElectricalType>,
    pub year_built: Option<i64>,
    pub effective_year: Option<i64>,
    pub remodel_year: Option<i64>,
    pub lot_size_sqft: Option<i64>,
    pub zoning_code: Option<String>,
    pub extra_features: Option<ExtraFeatures>,
    pub metadata: Option<Metadata>,
}
