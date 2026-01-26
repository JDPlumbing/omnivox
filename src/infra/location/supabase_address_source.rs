// infra/location/supabase_address_source.rs
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::supabasic::addresses::AddressRow;
use crate::shared::location::address_source::AddressSource;

pub struct SupabaseAddressSource {
    supa: Supabase,
}

impl SupabaseAddressSource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}

#[async_trait]
impl AddressSource for SupabaseAddressSource {
    async fn list(&self) -> Result<Vec<AddressRow>> {
        Ok(self
            .supa
            .from("addresses")
            .select("id, street_address, city, state, postal_code, country")
            .execute_typed::<AddressRow>()
            .await?)
    }

    async fn get(&self, id: Uuid) -> Result<Option<AddressRow>> {
        match self
            .supa
            .from("addresses")
            .select("*")
            .eq("id", &id.to_string())
            .single_typed::<AddressRow>()
            .await
        {
            Ok(row) => Ok(Some(row)),
            Err(_) => Ok(None),
        }
    }

    async fn create(&self, addr: AddressRow) -> Result<AddressRow> {
        AddressRow::create(&self.supa, &addr)
            .await
            .map_err(Into::into)

    }

    async fn update(&self, id: Uuid, update: serde_json::Value) -> Result<AddressRow> {
        let mut rows = self
            .supa
            .from("addresses")
            .eq("id", &id.to_string())
            .update(update)
            .select("*")
            .execute_typed::<AddressRow>()
            .await?;

        rows.pop().ok_or_else(|| anyhow!("address not found"))
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        self.supa
            .from("addresses")
            .eq("id", &id.to_string())
            .delete()
            .execute()
            .await?;
        Ok(())
    }
}
