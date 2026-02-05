use crate::core::render::primitives::RenderPrimitive;

use crate::core::simulation::state::SimulationState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::cosmic::systems::radiation_system::CosmicRadiationSystem;
use crate::core::math::vec3::Vec3;
use crate::core::tdt::sim_time::SimTime;
use crate::core::worlds::id::WorldId;

pub fn build(
    state: &SimulationState,
    world_id: WorldId,
    time: SimTime,
) -> Vec<RenderPrimitive> {
    let mut out = Vec::new();

    let anchor = state.world.anchors.get(&world_id).unwrap();
    let surface = state.world.surfaces.get(&world_id).unwrap();
    let body_id = anchor.body;

    let frames = CosmicFrameSystem {
        state: &state.cosmic,
    };

    let pose = frames.body_pose(body_id, time);
    let center = pose.position;

    let radius = match surface {
        crate::core::worlds::components::world_surface::WorldSurface::Spherical { radius } => {
            radius.0
        }
        _ => return out,
    };

    // 🌍 World sphere
    out.push(RenderPrimitive::Sphere {
        center: center.into(),
        radius,
        cosmic_body_id: body_id.0,
    });

    // 🧭 North pole axis (body frame Z+)
    let north = pose.orientation * Vec3::new(0.0, 0.0, 1.0);

    out.push(RenderPrimitive::Line {
        from: center.into(),
        to: (center + north * radius * 1.2).into(),
    });

    // 🌞 Subsolar point arrow
    if let Some(orbit) = state.cosmic.orbits.get(&body_id) {
        let star_id = orbit.primary;

        let radiation_system = CosmicRadiationSystem {
            state: &state.cosmic,
            frames: &frames,
        };

        if let Some(radiation) =
            radiation_system.radiation_from_body(star_id, body_id, time)
        {
            // Sun direction in cosmic frame
            let sun_dir_cosmic = radiation.direction.normalized();

            // Transform into body frame
            let sun_dir_body =
                pose.orientation.transpose() * sun_dir_cosmic;

            // Subsolar surface point (in cosmic space)
            let subsolar_surface =
                center + sun_dir_body.normalized() * radius;

            // Arrow pointing outward from subsolar point
            out.push(RenderPrimitive::Vector {
                origin: subsolar_surface.into(),
                direction: sun_dir_body.into(),
                scale: radius * 0.5,
            });


        }
    }

    out
}
