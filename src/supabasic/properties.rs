use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::core::WorldId;
use crate::core::UvoxId;
use crate::supabasic::geolocations::GeolocationRecord as GeoCoords;

// ------------------------
// Enums (core controlled vocab) 
// ------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyType {
    SingleFamily,
    MultiFamily,
    Condo,
    Townhome,
    MobileHome,
    Commercial,
    Industrial,
    Agricultural,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FoundationType {
    SlabOnGrade,
    Crawlspace,
    PierAndBeam,
    Basement,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExteriorType {
    Stucco,
    Brick,
    ConcreteBlock,
    WoodSiding,
    VinylSiding,
    Metal,
    Mixed,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RoofType {
    Gable,
    Hip,
    Flat,
    Mansard,
    Shed,
    Gambrel,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RoofMaterial {
    AsphaltShingle,
    Metal,
    Tile,
    Clay,
    Concrete,
    BuiltUp,
    Membrane,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlumbingType {
    Copper,
    PEX,
    CPVC,
    Galvanized,
    CastIron,
    Mixed,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ElectricalType {
    Aluminum,
    Copper,
    Mixed,
    Other,
}

// ------------------------
// JSON Fields (soft schema extensions)
// ------------------------

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtraFeatures {
    pub pool: Option<bool>,
    pub boat_dock: Option<bool>,
    pub solar_panels_kw: Option<f64>,
    pub irrigation_type: Option<String>,
    pub garage_spaces: Option<i64>,
    pub carport: Option<bool>,
    pub fireplace: Option<bool>,
    pub fence_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Metadata {
    pub import_source: Option<String>,      // e.g., "bcpa", "user_input"
    pub confidence_score: Option<f64>,      // how certain a scraper is
    pub last_scraped: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}


// ------------------------
// Main PropertyRecord
// ------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyRecord {
    // Core identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_id: Option<Uuid>,
  
    pub address_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,

    pub anchor_uvox: serde_json::Value,

    pub world_id: WorldId, // optional denormalization

    pub name: Option<String>,
    pub property_type: Option<PropertyType>,

    // Physical characteristics
    pub square_feet: Option<i64>,
    pub sqft_under_air: Option<i64>,
    pub bedrooms: Option<i64>,
    pub bathrooms: Option<i64>,
    pub num_units: Option<i64>,
    pub stories: Option<i64>,
    pub num_buildings: Option<i64>,
    pub ceiling_height_ft: Option<f64>,

    // Construction details
    pub foundation_type: Option<FoundationType>,
    pub exterior_type: Option<ExteriorType>,
    pub roof_type: Option<RoofType>,
    pub roof_material: Option<RoofMaterial>,
    pub plumbing: Option<PlumbingType>,
    pub electrical: Option<ElectricalType>,

    // Chronology
    pub year_built: Option<i64>,
    pub effective_year: Option<i64>,
    pub remodel_year: Option<i64>,

    // Lot / site info
    pub lot_size_sqft: Option<i64>,
    pub zoning_code: Option<String>,

    // Optional extensions
    pub extra_features: Option<ExtraFeatures>,
    pub metadata: Option<Metadata>,

    // System field
    pub created_at: Option<DateTime<Utc>>,
}

// ------------------------
// ORM Implementation
// ------------------------

impl DbModel for PropertyRecord {
    fn table() -> &'static str { "properties" }
}

impl PropertyRecord {
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("*")
            .execute_typed::<Self>()
            .await
    }

    pub async fn get(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("*")
            .eq("property_id", &id.to_string())
            .single_typed::<Self>()
            .await
    }

    pub async fn create(supa: &Supabase, payload: &Self) -> Result<Self, SupabasicError> {
        // ✅ 1. Check if a property already exists with the same address_id
        if let Some(addr_id) = payload.address_id {
            match supa
                .from(Self::table())
                .select("*")
                .eq("address_id", &addr_id.to_string())
                .maybe_single_typed::<Self>() // 👈 only returns Some(record) or None
                .await
            {
                Ok(Some(existing)) => {
                    eprintln!("🏠 Reusing existing property for address_id={}", addr_id);
                    return Ok(existing);
                }
                Ok(None) => eprintln!("ℹ️ No existing property found for address_id={}", addr_id),
                Err(e) => eprintln!("⚠️ Dedup check failed: {:?}", e),
            }
        }

        // ✅ 2. Otherwise insert a new record
        let raw = supa
            .from(Self::table())
            .insert(payload)
            .select("*")
            .execute()
            .await?;

        let inserted: Vec<Self> = serde_json::from_value(raw.clone())
            .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

        inserted
            .into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }

    pub async fn update(supa: &Supabase, id: Uuid, payload: &Self) -> Result<Self, SupabasicError> {
        let raw = supa
            .from(Self::table())
            .update(serde_json::to_value(payload)
                .map_err(|e| SupabasicError::Other(format!("serialization error: {e}")))?)
            .eq("property_id", &id.to_string())
            .select("*")
            .execute()
            .await?;

        let updated: Vec<Self> = serde_json::from_value(raw.clone())
            .map_err(|e| SupabasicError::Other(format!("decode error: {e:?}, raw={raw}")))?;

        updated
            .into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty update response".into()))
    }

    pub async fn delete(supa: &Supabase, id: Uuid) -> Result<(), SupabasicError> {
        supa.from(Self::table())
            .delete()
            .eq("property_id", &id.to_string())
            .execute()
            .await?;
        Ok(())
    }
}
