use uuid::Uuid;
use omnivox::supabasic::Supabase;

use omnivox::sim::{
    persist::spawn_entity_with_objex,
    world::SimWorld,
};
use omnivox::objex::{Objex, Shape, MaterialLink};
use omnivox::geospec::shapes::BoxShape;
use omnivox::chronovox::{ChronoEvent, EventKind, UvoxId};
use omnivox::tdt::core::TimeDelta;
use omnivox::uvoxxyz::Cartesian;

#[tokio::test]
async fn test_tick_spawns_and_moves() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
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
        name: "unit cube".into(),
        shape: Shape::Box(cube),
        material: MaterialLink::vacuum(),
    };

    let uvox = UvoxId::earth(0, 0, 0);

    // Persist entity + spawn
    let (_eid, _spawn_event) =
        spawn_entity_with_objex(&sup, sim_id, world.frame_id as i64, objex.clone(), uvox).await?;

    // Tick forward once (async now!)
    world.tick(Some(&sup)).await?;

    // Inject a manual move event (not persisted — just in-memory)
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

use omnivox::sim::world::SimWorld;
use omnivox::chronovox::{EventKind, UvoxId};
use omnivox::objex::{Objex, Shape, MaterialLink};
use omnivox::geospec::shapes::Sphere;

#[tokio::test]
async fn test_movement_system_generates_move() {
    let mut world = SimWorld::default();

    // Spawn a dummy object into the world
    let obj_id = UvoxId {
        frame_id: 0,
        r_um: 0,
        lat_code: 0,
        lon_code: 0,
    };
    world.objects.insert(obj_id, Objex {
        entity_id: uuid::Uuid::new_v4(),
        name: "moving_obj".into(),
        shape: Shape::Sphere(Sphere { radius: 1.0 }),
        material: MaterialLink::vacuum(),
    });

    // Tick once — MovementSystem should run (async now!)
    let events = world.tick(None).await.unwrap();

    println!("Events after tick: {:?}", events);

    // Assert we got at least one Move event
    assert!(
        events.iter().any(|e| matches!(e.kind, EventKind::Move { .. })),
        "Expected MovementSystem to emit a Move event"
    );
}
