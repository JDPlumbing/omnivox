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

//
// ──────────────────────────────────────────────────────────────
//   ROUTE HANDLERS (AppState-aware)
// ──────────────────────────────────────────────────────────────
//

pub async fn list_properties(State(app): State<AppState>) -> impl IntoResponse {
    match PropertyRecord::list(&app.supa).await {
        Ok(records) => Json(records).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

pub async fn list_properties_for_world(State(app): State<AppState>, Path(world_id): Path<i64>) -> impl IntoResponse {
    match app.supa
        .from("properties")
        .select("*")
        .eq("world_id", &world_id.to_string())
        .execute_typed::<PropertyRecord>()
        .await {
            Ok(rows) => Json(rows).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Fetch failed: {e:?}")).into_response()
    }
}


pub async fn get_property(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match PropertyRecord::get(&app.supa, id).await {
        Ok(record) => Json(record).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

pub async fn create_property(
    State(app): State<AppState>,
    Json(req): Json<PropertyInput>,
) -> impl IntoResponse {
    match PropertyRecord::create(&app.supa, &req.into()).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

pub async fn update_property(
    State(app): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<PropertyInput>,
) -> impl IntoResponse {
    match PropertyRecord::update(&app.supa, id, &req.into()).await {
        Ok(updated) => Json(updated).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}

pub async fn delete_property(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match PropertyRecord::delete(&app.supa, id).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "deleted": id }))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
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