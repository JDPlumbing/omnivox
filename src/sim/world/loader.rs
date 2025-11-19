use crate::supabasic::{Supabase};
use crate::supabasic::entity::EntityRecord;
use crate::supabasic::objex::ObjexRecord as BlueprintRecord;
use crate::sim::entity::SimEntity;
use crate::sim::world::WorldState;

pub async fn load_world(supa: &Supabase, frame_id: i64) -> anyhow::Result<WorldState> {
    // 0. Load world metadata
    let meta = crate::supabasic::worlds::WorldRecord::fetch(supa, frame_id).await?;

    // 1. Load entity rows
    let records = EntityRecord::list_for_frame(supa, frame_id).await?;

    // 2. Load blueprint rows ONCE
    let mut cache = std::collections::HashMap::new();
    for rec in &records {
        if !cache.contains_key(&rec.blueprint_id) {
            let bp = BlueprintRecord::fetch(supa, rec.blueprint_id).await?;
            cache.insert(rec.blueprint_id, bp.into_blueprint());
        }
    }

    // 3. Convert to SimEntity
    let entities: Vec<SimEntity> = records
        .into_iter()
        .map(|r| {
            let bp = cache.get(&r.blueprint_id).unwrap().clone();
            r.into_sim_entity(bp)
        })
        .collect();

    // 4. WorldState
    Ok(WorldState::from_entities(meta, entities))
}
