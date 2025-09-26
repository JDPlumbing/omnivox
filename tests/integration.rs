use omnivox::sim::SimWorld;
use chronovox::{EventKind, UvoxId};

#[test]
fn test_bootstrap_world_has_earth() {
    // Build a fresh world
    let world = SimWorld::bootstrap_world();

    // Assert Earth object exists in objects
    let earth = world.objects.values().find(|o| o.name == "Earth");
    assert!(earth.is_some(), "Earth object should be present in world.objects");

    // Assert a Spawn event exists at tick 0
    let spawn = world.timeline.iter_chronological().find(|e| {
        matches!(e.kind, EventKind::Spawn)
            && e.t.ticks("nanoseconds") == 0
            && e.id == UvoxId::earth(0, 0, 0)
    });
    assert!(spawn.is_some(), "Earth should have a Spawn event at tick 0");
}
