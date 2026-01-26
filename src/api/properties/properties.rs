use axum::{
    //debug_handler,
    extract::{Path, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
    use crate::core::{ WorldId, UvoxId};

use crate::{
    shared::app_state::AppState, // 👈 new import
    supabasic::{
        SupabasicError,
        properties::{
            PropertyRecord,
            PropertyType,
            FoundationType,
            ExteriorType,
            RoofType,
            RoofMaterial,
            PlumbingType,
            ElectricalType,
            ExtraFeatures,
            Metadata,
        },
    },
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorDto {
    pub world_id: WorldId,
    pub uvox: UvoxId,
}

/// Payload for creating or updating a property
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

impl From<PropertyInput> for PropertyRecord {
    fn from(input: PropertyInput) -> Self {
        Self {
            property_id: None,
            address_id: input.address_id,
            world_id: input.anchor.world_id,

            anchor_uvox: serde_json::to_value(input.anchor.uvox)
                .expect("UvoxId must serialize"),


            user_owner_id: input.user_owner_id,
            name: input.name,
           
            property_type: input.property_type,
            square_feet: input.square_feet,
            sqft_under_air: input.sqft_under_air,
            bedrooms: input.bedrooms,
            bathrooms: input.bathrooms,
            num_units: input.num_units,
            stories: input.stories,
            num_buildings: input.num_buildings,
            ceiling_height_ft: input.ceiling_height_ft,
            foundation_type: input.foundation_type,
            exterior_type: input.exterior_type,
            roof_type: input.roof_type,
            roof_material: input.roof_material,
            plumbing: input.plumbing,
            electrical: input.electrical,
            year_built: input.year_built,
            effective_year: input.effective_year,
            remodel_year: input.remodel_year,
            lot_size_sqft: input.lot_size_sqft,
            zoning_code: input.zoning_code,
            extra_features: input.extra_features,
            metadata: input.metadata,
            created_at: None,
        }
    }
}









//TODO make this for entities instead of objex
/*
use crate::sim::generators::property_objex::generate_property_objexes;
use crate::supabasic::objex::ObjexRecord;

pub async fn generate_property_objects(
    State(app): State<AppState>,
    Path(property_id): Path<Uuid>,
) -> impl IntoResponse {
    // 1️⃣ Fetch the property
    let property = match PropertyRecord::get(&app.supa, property_id).await {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("property not found: {e:?}") })),
            )
                .into_response();
        }
    };

    // 2️⃣ Delete any existing Objex for this property
    match app
        .supa
        .from("objex_entities")
        .eq("property_id", &property_id.to_string())
        .delete()
        .execute()
        .await
    {
        Ok(_) => eprintln!("🧹 Cleared old Objex for property {property_id}"),
        Err(e) => eprintln!("⚠️ Failed to clear old Objex: {:?}", e),
    }

    // 3️⃣ Generate new Objex from property data
    let objs = generate_property_objexes(&property);
    if objs.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "no objects generated for property" })),
        )
            .into_response();
    }

    // 4️⃣ Convert and insert
    let records: Vec<ObjexRecord> = objs.into_iter().map(ObjexRecord::from).collect();

    match ObjexRecord::create_many(&app.supa, &records).await {
        Ok(created) => Json(json!({
            "status": "ok",
            "generated_count": created.len(),
            "property_id": property_id,
            "entities": created,
            "replaced_old": true
        }))
        .into_response(),

        Err(e) => {
            eprintln!("⚠️ Bulk insert failed: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}
*/