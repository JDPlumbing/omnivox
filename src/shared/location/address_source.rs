// shared/location/address_source.rs
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::supabasic::addresses::AddressRow;

#[async_trait]
pub trait AddressSource: Send + Sync {
    async fn list(&self) -> Result<Vec<AddressRow>>;
    async fn get(&self, id: Uuid) -> Result<Option<AddressRow>>;
    async fn create(&self, addr: AddressRow) -> Result<AddressRow>;
    async fn update(&self, id: Uuid, update: serde_json::Value) -> Result<AddressRow>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
