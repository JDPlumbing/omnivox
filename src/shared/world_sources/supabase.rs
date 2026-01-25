use anyhow::Result;
use async_trait::async_trait;
use anyhow::anyhow;
use crate::supabasic::Supabase;
use crate::supabasic::worlds::{ WorldRow, NewWorldRow };
use crate::supabasic::entity::EntityRow;

use crate::core::id::{WorldId, EntityId};
use crate::core::tdt::sim_time::SimTime;
use crate::core::UvoxId;

use crate::engine::world::state::{
    WorldState,
};
use crate::core::components::{
    position::Position,
    orientation::Orientation,
    lifecycle::Lifecycle,
};

use crate::core::world::World;
use crate::core::world::WorldEnvironment;
use crate::core::world::presets::earth_v0;
use crate::core::world::WorldStats;


use super::source::WorldSource;

pub struct SupabaseWorldSource {
    pub supa: Supabase,
}

#[async_trait]
impl WorldSource for SupabaseWorldSource {
    async fn list_worlds(&self) -> Result<Vec<WorldRow>> {
        let worlds = WorldRow::list(&self.supa).await?;
        Ok(worlds)
    }
    async fn get_world(&self, world_id: WorldId) -> Result<WorldRow> {
        let world = WorldRow::get(&self.supa, world_id).await?;
        Ok(world)
    }
    async fn create_world(&self, payload: NewWorldRow) -> Result<WorldRow> {
        let world = WorldRow::create(&self.supa, &payload).await?;
        Ok(world)
    }
    async fn update_world(
        &self,
        world_id: WorldId,
        changes: serde_json::Value,
    ) -> Result<WorldRow> {
        let mut rows = self.supa
            .from("worlds")
            .update(changes)
            .eq("world_id", &world_id.to_string())
            .select("*")
            .execute_typed::<WorldRow>()
            .await?;

        rows.pop().ok_or_else(|| anyhow!("world not found"))
    }

    async fn delete_world(&self, world_id: WorldId) -> Result<()> {
        WorldRow::delete(&self.supa, world_id).await?;
        Ok(())
    }

    async fn world_stats(&self, world_id: WorldId) -> Result<WorldStats> {
        let args = serde_json::json!({
            "p_world_id": world_id.to_string()
        });

        let val = self.supa
            .rpc("count_sim_entities", args)
            .await?;

        Ok(WorldStats {
            world_id,
            entity_count: val.as_u64().unwrap_or(0),
        })
    }

    async fn set_world_epoch(
        &self,
        world_id: WorldId,
        epoch: SimTime,
    ) -> Result<()> {
        self.supa
            .from("worlds")
            .update(serde_json::json!({
                "world_epoch": epoch.as_ns().to_string()
            }))
            .eq("world_id", &world_id.to_string())
            .execute()
            .await?;

        Ok(())
    }


    async fn load_world(&self, world_id: WorldId) -> Result<WorldState> {
        //
        // 1. Load world metadata
        //
        let meta_rec = WorldRow::get(&self.supa, world_id).await?;

        let env_desc = meta_rec.environment
            .clone()
            .unwrap_or_else(|| {
                log::warn!(
                    "World {} has no environment, defaulting to earth_v0",
                    world_id
                );
                earth_v0()
            });

        let world_env = WorldEnvironment::from_descriptor(&env_desc);

        let meta = World {
            id: world_id,
            name: meta_rec.name.clone(),
            description: meta_rec.description.clone(),
            world_epoch: meta_rec.world_epoch
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .map(SimTime::from_ns),
        };

        let mut state = WorldState::new(meta, world_env);

        //
        // 2. Load entity rows
        //
        let rows: Vec<EntityRow> =
            EntityRow::list_for_world(&self.supa, world_id).await?;

        for row in rows {
            let id = EntityId(row.row_id.expect("EntityRow missing row_id"));

            // Register entity
            state.entities.insert(id);
            state.world_membership.insert(id, world_id);

            // Position (mandatory)
            let position: UvoxId =
                serde_json::from_value(row.position)
                    .expect("Invalid UvoxId in DB");

            state.positions.insert(id, Position(position));

            // Orientation (optional)
            if !row.orientation.is_null() {
                let orientation =
                    serde_json::from_value(row.orientation)
                        .expect("Invalid orientation in DB");

                state.orientations.insert(id, Orientation(orientation));
            }

            // Lifecycle
            state.lifecycles.insert(id, Lifecycle {
                spawned_at: row.spawned_at,
                despawned_at: row.despawned_at,
            });

            // Metadata
            state.metadata.insert(id, row.metadata);
        }

        Ok(state)
    }
}
