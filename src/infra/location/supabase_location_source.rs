use async_trait::async_trait;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use serde_json::Value;

use crate::supabasic::Supabase;
use crate::supabasic::addresses::AddressRow;

use crate::shared::location::{
    location_source::LocationSource,
    resolved_location::ResolvedLocation,
};

use crate::core::{WorldId, UvoxId};
use crate::core::uvoxid::{LatCode, LonCode, RUm, EARTH_RADIUS_UM};

pub struct SupabaseLocationSource {
    supa: Supabase,
    http: reqwest::Client,
}

impl SupabaseLocationSource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
            http: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl LocationSource for SupabaseLocationSource {
    async fn resolve_address(
        &self,
        address_id: Uuid,
    ) -> Result<ResolvedLocation> {

        // 1️⃣ Fetch address
        let address: AddressRow = self
            .supa
            .from("addresses")
            .select("id, street_address, city, state, country")
            .eq("id", &address_id.to_string())
            .single_typed()
            .await
            .map_err(|_| anyhow!("address not found"))?;

        // 2️⃣ Existing geolocation?
        if let Ok(existing) = self
            .supa
            .from("geolocations")
            .select("id, lat, lon, elevation_m")
            .eq("address_id", &address_id.to_string())
            .single()
            .await
        {
            return self.resolve_existing(address_id, existing).await;
        }

        // 3️⃣ External geocoding
        let (lat, lon, elevation_m) =
            self.geocode_address(&address).await?;

        // 4️⃣ Insert geolocation
        let geo = self
            .supa
            .from("geolocations")
            .insert(serde_json::json!({
                "address_id": address_id,
                "lat": lat,
                "lon": lon,
                "elevation_m": elevation_m,
            }))
            .select("id, lat, lon, elevation_m")
            .single()
            .await?;

        // 5️⃣ Resolve or create uvox
        let uvox = self.resolve_uvox(lat, lon, elevation_m).await?;

        Ok(ResolvedLocation {
            address_id,
            geolocation_id: geo["id"].as_str().unwrap().parse()?,
            lat,
            lon,
            elevation_m,
            uvox,
            world_id: WorldId(0), // 🌍 Earth frame (default)
            reused: false,
        })
    }
}

impl SupabaseLocationSource {

    async fn geocode_address(
        &self,
        addr: &AddressRow,
    ) -> Result<(f64, f64, f64)> {

        let query = format!(
            "{}, {}, {}, {}",
            addr.street_address.clone().unwrap_or_default(),
            addr.city.clone().unwrap_or_default(),
            addr.state.clone().unwrap_or_default(),
            addr.country.clone().unwrap_or_default(),
        );

        let api_key = std::env::var("OPENCAGE_API_KEY")
            .map_err(|_| anyhow!("OPENCAGE_API_KEY missing"))?;

        let url = format!(
            "https://api.opencagedata.com/geocode/v1/json?q={}&key={}",
            urlencoding::encode(&query),
            api_key
        );

        let resp: Value = self.http
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        let result = resp["results"]
            .get(0)
            .ok_or_else(|| anyhow!("no geocode results"))?;

        let lat = result["geometry"]["lat"]
            .as_f64()
            .ok_or_else(|| anyhow!("missing lat"))?;

        let lon = result["geometry"]["lng"]
            .as_f64()
            .ok_or_else(|| anyhow!("missing lon"))?;

        Ok((lat, lon, 0.0))
    }

    async fn resolve_uvox(
        &self,
        lat: f64,
        lon: f64,
        elevation_m: f64,
    ) -> Result<UvoxId> {

        let r_um =
            ((EARTH_RADIUS_UM as f64 / 1_000_000.0 + elevation_m) * 1_000_000.0) as i64;

        let lat_code = LatCode::from_degrees(lat);
        let lon_code = LonCode::from_degrees(lon);

        Ok(UvoxId::new(
            RUm(r_um),
            lat_code,
            lon_code,
        ))
    }

    async fn resolve_existing(
        &self,
        address_id: Uuid,
        geo: Value,
    ) -> Result<ResolvedLocation> {

        let lat = geo["lat"].as_f64().unwrap_or(0.0);
        let lon = geo["lon"].as_f64().unwrap_or(0.0);
        let elevation_m = geo["elevation_m"].as_f64().unwrap_or(0.0);

        let uvox = self.resolve_uvox(lat, lon, elevation_m).await?;

        Ok(ResolvedLocation {
            address_id,
            geolocation_id: geo["id"].as_str().unwrap().parse()?,
            lat,
            lon,
            elevation_m,
            uvox,
            world_id: WorldId(0),
            reused: true,
        })
    }
}
