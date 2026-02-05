use crate::core::render::primitives::RenderPrimitive;
use crate::core::simulation::state::SimulationState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::tdt::sim_time::SimTime;
use crate::core::math::vec3::Vec3;

pub fn build(
    state: &SimulationState,
    time: SimTime,
) -> Vec<RenderPrimitive> {
    let mut out = Vec::new();

    let frame = CosmicFrameSystem {
        state: &state.cosmic,
    };

    // Render every body that has a radius
    for (body_id, radius) in &state.cosmic.radii {
        let pose = frame.body_pose(*body_id, time);

        let center = pose.position;
        let r = radius.meters.0;

        // -----------------------------
        // Body sphere
        // -----------------------------
        out.push(RenderPrimitive::Sphere {
            center: center.into(),
            radius: r,
            cosmic_body_id: body_id.0,
        });
        // -----------------------------
        // Body light (if any)
        // -----------------------------
        if let Some(lum) = state.cosmic.luminosities.get(body_id) {
            out.push(RenderPrimitive::PointLight {
                position: pose.position.into(),
                intensity: lum.watts.0,
            });
        }

        // -----------------------------
        // Debug: north / spin axis
        // -----------------------------
        // Local +Z is canonical "north" in body space
        let north_local = Vec3::new(0.0, 0.0, 1.0);

        // Rotate into cosmic frame
        let north_world = pose.orientation * north_local;

        // Draw axis line (slightly longer than radius)
        out.push(RenderPrimitive::Line {
            from: center.into(),
            to: (center + north_world * r * 1.2).into(),
        });
    }

    out
}
