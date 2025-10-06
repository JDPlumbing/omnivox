use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use serde::{Serialize, Deserialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeolocationRecord {
    pub id: Option<Uuid>,
    pub address_id: Option<Uuid>,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: Option<f64>,
}

impl DbModel for GeolocationRecord {
    fn table() -> &'static str { "geolocations" }
}

impl GeolocationRecord {
    /// Create a new geolocation row
    pub async fn create(supa: &Supabase, geo: &Self) -> Result<Self, SupabasicError> {
        // ✅ Build JSON array explicitly
        let json_body = json!([{
            "address_id": geo.address_id,
            "lat": geo.lat,
            "lon": geo.lon,
            "elevation_m": geo.elevation_m
        }]);

        println!("DEBUG geolocation insert body: {}", serde_json::to_string_pretty(&json_body).unwrap());

        // ✅ Execute insert with select
        let res = supa
            .from(Self::table())
            .insert(json_body)
            .select("id, address_id, lat, lon, elevation_m")
            .execute()
            .await?;

        println!("DEBUG geolocation insert result: {}", serde_json::to_string_pretty(&res).unwrap());

        // ✅ Deserialize into typed Vec
        let inserted: Vec<Self> = serde_json::from_value(res)
            .map_err(|e| SupabasicError::Other(format!("Deserialize error: {e:?}")))?;

        inserted
            .into_iter()
            .next()
            .ok_or_else(|| SupabasicError::Other("empty insert response".into()))
    }

    /// Optionally: fetch all geolocations
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id, address_id, lat, lon, elevation_m")
            .execute_typed::<Self>()
            .await
    }

    /// Optionally: fetch one by address_id
    pub async fn get_by_address_id(supa: &Supabase, address_id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("id, address_id, lat, lon, elevation_m")
            .eq("address_id", &address_id.to_string())
            .single_typed::<Self>()
            .await
    }
}
