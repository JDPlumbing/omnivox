use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::location::address::{Address, CreateAddress, UpdateAddress};
use crate::shared::location::address_source::AddressSource;

/// --------------------------------------------------
/// In-memory AddressSource
/// --------------------------------------------------

pub struct InMemoryAddressSource {
    addresses: Mutex<HashMap<Uuid, Address>>,
}

impl Default for InMemoryAddressSource {
    fn default() -> Self {
        Self {
            addresses: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AddressSource for InMemoryAddressSource {
    // ----------------------------
    // Queries
    // ----------------------------

    async fn list(&self) -> Result<Vec<Address>> {
        let map = self.addresses.lock().unwrap();
        Ok(map.values().cloned().collect())
    }

    async fn get(&self, id: Uuid) -> Result<Option<Address>> {
        let map = self.addresses.lock().unwrap();
        Ok(map.get(&id).cloned())
    }

    // ----------------------------
    // Commands
    // ----------------------------

    async fn create(&self, cmd: CreateAddress) -> Result<Address> {
        let mut map = self.addresses.lock().unwrap();

        let id = Uuid::new_v4();

        let address = Address {
            id: Some(id),
            street_address: cmd.street_address,
            city: cmd.city,
            state: cmd.state,
            postal_code: cmd.postal_code,
            country: cmd.country,
        };

        map.insert(id, address.clone());
        Ok(address)
    }

    async fn update(&self, cmd: UpdateAddress) -> Result<Address> {
        let mut map = self.addresses.lock().unwrap();

        let id = cmd.id;

        let addr = map
            .get_mut(&id)
            .ok_or_else(|| anyhow!("Address not found"))?;

        if let Some(v) = cmd.street_address {
            addr.street_address = v;
        }
        if let Some(v) = cmd.city {
            addr.city = Some(v);
        }
        if let Some(v) = cmd.state {
            addr.state = Some(v);
        }
        if let Some(v) = cmd.postal_code {
            addr.postal_code = Some(v);
        }
        if let Some(v) = cmd.country {
            addr.country = Some(v);
        }

        Ok(addr.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut map = self.addresses.lock().unwrap();
        map.remove(&id);
        Ok(())
    }
}
