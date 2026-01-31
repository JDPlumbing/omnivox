use crate::core::math::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct EnuFrame {
    pub east: Vec3,
    pub north: Vec3,
    pub up: Vec3,
}

/// Construct an ENU frame given:
/// - `up`: surface normal (need not be normalized)
/// - `north_hint`: a direction indicating "north" (need not be perpendicular)
///
/// This function performs only geometry, no semantic interpretation.
pub fn enu_frame(up: Vec3, north_hint: Vec3) -> EnuFrame {
    let up = up.normalized();

    // Remove any component of north_hint along up
    let north_proj = north_hint - up * north_hint.dot(up);

    let north = north_proj.normalized();
    let east = north.cross(up);

    EnuFrame {
        east,
        north,
        up,
    }
}
