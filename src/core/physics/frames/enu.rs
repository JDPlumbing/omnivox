use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::tides::AnchorError;
use crate::core::math::vec3::{dot, normalize, magnitude, cross};
//use crate::core::world::world_frame::FrameModel;
//use crate::core::math::mat3::rotate_around_axis;

/// Local East-North-Up frame in world coordinates
#[derive(Debug, Clone, Copy)]
pub struct ENUFrame {
    pub east:  [f64; 3],
    pub north: [f64; 3],
    pub up:    [f64; 3],
}

impl ENUFrame {
    /// Project a world-space vector into ENU components
    pub fn project(&self, v: [f64; 3]) -> [f64; 3] {
        [
            dot(self.east,  v),
            dot(self.north, v),
            dot(self.up,    v),
        ]
    }
}

/// Construct the ENU frame at a surface point
pub fn enu_frame(
    resolver: &WorldResolver,
    world: WorldId,
    surface: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<ENUFrame, AnchorError> {
    let surface_pos = resolver
    .world_anchor_point(world, surface, time, space)
    .map_err(AnchorError::from)?;

let center_pos = resolver.world_pose(world, time).position_m;

// Up: outward radial direction
let up = normalize([
    surface_pos[0] - center_pos[0],
    surface_pos[1] - center_pos[1],
    surface_pos[2] - center_pos[2],
]);

// Spin axis in world space (already tilted)
let pose = resolver.world_pose(world, time);
let spin_axis = normalize(pose.orientation * [0.0, 0.0, 1.0]);

// East: tangent to latitude circle
let mut east = cross(spin_axis, up);
let east_mag = magnitude(east);
if east_mag < 1e-12 {
    return Err(AnchorError::Singularity); // pole
}
east = [
    east[0] / east_mag,
    east[1] / east_mag,
    east[2] / east_mag,
];

// North: completes right-handed frame
let north = cross(up, east);

Ok(ENUFrame { east, north, up })
}