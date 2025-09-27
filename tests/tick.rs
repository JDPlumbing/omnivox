// tests/tick.rs
use uuid::Uuid;
use supabasic::Supabase;

use omnivox::sim::{
    persist::spawn_entity_with_objex,
    world::SimWorld,
};
use objex::Objex;
use geospec::shapes::{BoxShape};
use chronovox::{ChronoEvent, EventKind, UvoxId};
use tdt::core::TimeDelta;
use geospec::coords::Cartesian;

#[tokio::test]
async fn test_tick_spawns_and_moves() -> anyhow::Result<()> {
    let url = std::env::var("SUPABASE_URL")?;
    let key = std::env::var("SUPABASE_KEY")?;
    let sup = Supabase::new(&url, &key);

    // Load the base world
    let sim_id = Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")?;
    let mut world = SimWorld::load_from_supabase(&sup, sim_id).await?;

    // --- Explicit cube object, no `Default`
    let cube = BoxShape {
        length: 1.0,
        width: 1.0,
        height: 1.0,
    };

    let entity_id = Uuid::new_v4();
    let objex = Objex {
        entity_id,
        name: Some("unit cube".into()),
        shape: cube.into(),
        material: "vacuum".into(), // minimal stub
    };

    let uvox = UvoxId::earth(0, 0, 0);

    // Persist entity + spawn
    let (_eid, _spawn_event) =
        spawn_entity_with_objex(&sup, sim_id, world.frame_id as i64, objex.clone(), uvox).await?;

    // Tick forward once
    world.tick(1);

    // Inject a manual move event
    let move_event = ChronoEvent {
        id: uvox,
        t: TimeDelta::from_ticks(1, "nanoseconds"),
        kind: EventKind::Move {
            offset: Cartesian { x: 1.0, y: 0.0, z: 0.0 },
        },
        payload: None,
    };
    world.timeline.events.push(move_event);

    // Reload to see persisted timeline
    let world_after = SimWorld::load_from_supabase(&sup, sim_id).await?;

    println!("TIMELINE AFTER TICK: {:?}", world_after.timeline.events);

    assert!(
        world_after.timeline.events.len() > 1,
        "Expected >1 events (spawn + move)"
    );

    Ok(())
}
