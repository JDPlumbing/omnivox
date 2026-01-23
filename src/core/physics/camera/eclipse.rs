// core/physics/camera/eclipse.rs

use super::project::CameraVector;
use crate::core::{world::WorldResolver, 
    WorldId, 
    UvoxId, 
    world::world_env_descriptor::WorldSpace, 
    CameraPose, 
    SimTime, 
    //SimDuration, 
    EclipseError, 
    local_tangent_frame, 
    camera_basis_from_enu,
    project_world_dir_to_camera,
 };
use crate::core::math::vec3::{magnitude, normalize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EclipseType {
    None,
    Partial,
    Annular,
    Total,
}

#[derive(Debug, Clone, Copy)]
pub struct EclipseResult {
    pub eclipse: EclipseType,

    /// Angular separation between centers (radians)
    pub center_separation_rad: f64,

    /// Angular radii
    pub primary_radius_rad: f64,
    pub occluder_radius_rad: f64,
}

#[inline]
fn angular_offset(v: CameraVector) -> (f64, f64) {
    // angular offsets from camera forward axis
    let theta_x = v.x.atan2(v.z);
    let theta_y = v.y.atan2(v.z);
    (theta_x, theta_y)
}

pub fn test_disk_overlap(
    primary_cam: CameraVector,
    primary_radius_rad: f64,
    occluder_cam: CameraVector,
    occluder_radius_rad: f64,
) -> EclipseResult {
    // If occluder is not in front, no eclipse
    if occluder_cam.z >= primary_cam.z {
        return EclipseResult {
            eclipse: EclipseType::None,
            center_separation_rad: 0.0,
            primary_radius_rad,
            occluder_radius_rad,
        };
    }

    let (px, py) = angular_offset(primary_cam);
    let (ox, oy) = angular_offset(occluder_cam);

    let dx = px - ox;
    let dy = py - oy;

    let center_sep = (dx * dx + dy * dy).sqrt();

    let r_p = primary_radius_rad;
    let r_o = occluder_radius_rad;

    let eclipse = if center_sep >= r_p + r_o {
        EclipseType::None
    } else if center_sep <= (r_p - r_o).abs() {
        if r_o >= r_p {
            EclipseType::Total
        } else {
            EclipseType::Annular
        }
    } else {
        EclipseType::Partial
    };

    EclipseResult {
        eclipse,
        center_separation_rad: center_sep,
        primary_radius_rad: r_p,
        occluder_radius_rad: r_o,
    }
}


pub fn compute_eclipse_at_time(
    resolver: &WorldResolver,
    observer_world: WorldId,
    observer_uvox: &UvoxId,
    space: &WorldSpace,
    camera_pose: CameraPose,
    time: SimTime,
) -> Result<EclipseResult, EclipseError> {

    // Local tangent frame
    let frame = local_tangent_frame(
        resolver,
        observer_world,
        observer_uvox,
        time,
        space,
    )?;

    let basis = camera_basis_from_enu(frame.enu, camera_pose);
    let origin = frame.origin;

    let sun = WorldId(0);
    let moon = WorldId(2);

    // --- project bodies into camera space ---
let project_body = |body: WorldId| -> Result<(CameraVector, f64), EclipseError> {
    let pos = resolver.world_pose(body, time).position_m;

    let v = [
        pos[0] - origin[0],
        pos[1] - origin[1],
        pos[2] - origin[2],
    ];

    let dist = magnitude(v);
    let dir = normalize(v);

    Ok((project_world_dir_to_camera(basis, dir), dist))
};

let (sun_cam, sun_dist) = project_body(sun)?;
let (moon_cam, moon_dist) = project_body(moon)?;



    // --- angular radii ---
    let sun_radius_m = resolver.frames[&sun]
        .physical_radius_m
        .ok_or(EclipseError::NoPhysicalRadius)?;

    let moon_radius_m = resolver.frames[&moon]
        .physical_radius_m
        .ok_or(EclipseError::NoPhysicalRadius)?;

    let sun_radius_rad  = (sun_radius_m  / sun_dist).atan();
    let moon_radius_rad = (moon_radius_m / moon_dist).atan();

    // --- overlap test ---
    let overlap = test_disk_overlap(
        sun_cam,
        sun_radius_rad,
        moon_cam,
        moon_radius_rad,
    );

    Ok(EclipseResult {
        eclipse: overlap.eclipse,
        center_separation_rad: overlap.center_separation_rad,
        primary_radius_rad: sun_radius_rad,
        occluder_radius_rad: moon_radius_rad,
    })

}
