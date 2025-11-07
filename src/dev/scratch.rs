use omnivox::{
    sim::{
        systems::{collision::CollisionSystem, fracture::FractureSystem, movement::MovementSystem, System},
        world::WorldState,
        components::Velocity,
    },
    objex::core::{Objex, MaterialLink, MaterialName, Shape},
    matcat::materials::MatCatId,
};
use uuid::Uuid;

fn main() {
    const EARTH_RADIUS: i64 = 6_371_000_000_000;
    let mut world = WorldState::default();
    let mut systems: Vec<Box<dyn System>> = vec![
        Box::new(MovementSystem),
        Box::new(CollisionSystem),
        Box::new(FractureSystem),
        // add MovementSystem later if you want them to drift
    ];

    // --- setup object ---
    let obj_id = Uuid::new_v4().to_string();
    let material = MaterialLink::new(MaterialName::Copper);
    let shape = Shape::Sphere(omnivox::geospec::shapes::Sphere { radius: 1.0 });
    let mut obj = Objex::new(0, None, "test_ball", shape, material);
    obj.uvoxid.r_um = EARTH_RADIUS - 10;
    obj.material.matcat_id = Some(MatCatId::metal_cu());
    world.objects.insert(obj_id.clone(), obj);
    world.velocity_components.insert(
        Uuid::parse_str(&obj_id).unwrap(),
        Velocity { dr: -100.0, dlat: 0.0, dlon: 0.0 },
    );

    // --- run multiple ticks ---
    for tick in 0..5 {
        println!("=== Tick {tick} ===");
        for system in systems.iter_mut() {
            let events = system.tick(&mut world);
            if !events.is_empty() {
                println!("{events:#?}");
            }
        }
    }
}
