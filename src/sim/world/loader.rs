// src/sim/world/loader.rs

use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRecord;
use crate::supabasic::entity::EntityRecord;
use crate::core::SimTime;
use crate::sim::entity::SimEntity;
use crate::sim::world::state::{World, WorldState};

use anyhow::Result;

pub async fn load_world(
    supa: &Supabase,
    world_id: i64,
) -> Result<WorldState> {

    // 1. Load world metadata
    let meta_rec = WorldRecord::fetch(supa, world_id).await?;

    let meta = World {
        world_id: meta_rec.world_id,    
        name: meta_rec.name.clone(),
        description: meta_rec.description.clone(),
        world_epoch: meta_rec.world_epoch.map(SimTime::from_ns),
    };

    // 2. Fetch all entity rows for this world
    let rows: Vec<EntityRecord> =
        EntityRecord::list_for_world(supa, world_id).await?;

    // 3. Convert DB rows → SimEntity
    let mut entities = Vec::<SimEntity>::new();
    for row in rows {
        let sim = row.try_into()?;  // uses TryFrom<EntityRecord> for SimEntity
        entities.push(sim);
    }

    // 4. Return world state
    Ok(WorldState::from_entities(meta, entities))
}
