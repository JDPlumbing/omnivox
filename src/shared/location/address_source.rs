// shared/location/address_source.rs
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::core::spatial::location::{Address, CreateAddress, UpdateAddress};

#[async_trait]
pub trait AddressSource: Send + Sync {
    async fn list(&self) -> Result<Vec<Address>>;
    async fn get(&self, id: Uuid) -> Result<Option<Address>>;
    async fn create(&self, cmd: CreateAddress) -> Result<Address>;
    async fn update(&self, cmd: UpdateAddress) -> Result<Address>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
