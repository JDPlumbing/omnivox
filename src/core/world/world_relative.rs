
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::world_env_descriptor::WorldSpace;

pub fn world_to_world_vector(
    resolver: &WorldResolver,
    from_world: WorldId,
    from_uvox: &UvoxId,
    to_world: WorldId,
    time: SimTime,
    from_space: &WorldSpace,
) -> [f64; 3] {
    let from_pos = resolver.world_point(
        from_world,
        from_uvox,
        time,
        from_space,
    );

    let to_pos = resolver.world_origin(
        to_world,
        time,
    );

    [
        to_pos[0] - from_pos[0],
        to_pos[1] - from_pos[1],
        to_pos[2] - from_pos[2],
    ]
}
pub fn world_point_absolute(
    resolver: &WorldResolver,
    world_id: WorldId,
    uvox: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> [f64; 3] {
    resolver.world_point(world_id, uvox, time, space)
}
