use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::supabasic::{Supabase, SupabasicError};
use crate::supabasic::orm::DbModel;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressRow {
    pub id: Option<Uuid>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}


impl DbModel for AddressRow {
    fn table() -> &'static str { "addresses" }
}

impl AddressRow {
pub async fn create(supa: &Supabase, row: &Self) -> Result<Self, SupabasicError> {
    // MUST NOT BE WRAPPED IN AN ARRAY VVVV
    let payload = json!({
        "street_address": row.street_address,
        "city": row.city,
        "state": row.state,
        "postal_code": row.postal_code,
        "country": row.country
    });

    eprintln!("🔎 create_address DB call with payload: {:?}", payload);

    let mut result = supa
        .from("addresses")
        .insert(payload)
        .execute_typed::<AddressRow>()
        .await?;

    Ok(result.remove(0)) // one array layer, not two
}


    /// Lists all addresses
    pub async fn list(supa: &Supabase) -> Result<Vec<Self>, SupabasicError> {
        supa.from(Self::table())
            .select("id, street_address, city, state, postal_code, country")
            .execute_typed::<Self>()
            .await
    }

    /// Gets a single address by UUID
    pub async fn get(supa: &Supabase, id: Uuid) -> Result<Self, SupabasicError> {
        supa.from(Self::table())
            .select("id, street_address, city, state, postal_code, country")
            .eq("id", &id.to_string())
            .single_typed::<Self>()
            .await
    }

        /// Updates a full address (PUT)
    pub async fn update(supa: &Supabase, id: Uuid, updated: &Self) -> Result<Self, SupabasicError> {
        let payload = serde_json::to_value(updated)?;
        let mut result = supa
            .from(Self::table())
            .eq("id", &id.to_string())
            .update(payload)
            .select("*")
            .execute_typed::<Self>()
            .await?;
        Ok(result.remove(0))
    }


    /// Patches partial fields (PATCH)
    pub async fn patch(supa: &Supabase, id: Uuid, changes: serde_json::Value) -> Result<Self, SupabasicError> {
        let result = supa
            .from(Self::table())
            .eq("id", &id.to_string())
            .update(changes)
            .select("*")
            .execute_one::<Self>() // 👈 same here
            .await?;
        Ok(result)
    }

}
