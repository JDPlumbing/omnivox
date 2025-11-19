use crate::uvoxid::UvoxId;
use crate::uvoxxyz::types::Cartesian;
use crate::uvoxxyz::convert::UvoxIdExt;

/// Cross product of two vectors
fn cross(a: Cartesian, b: Cartesian) -> Cartesian {
    Cartesian {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

/// Normalize a vector
fn normalize(v: Cartesian) -> Cartesian {
    let norm = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    Cartesian { x: v.x / norm, y: v.y / norm, z: v.z / norm }
}

/// Build an ENU basis (East, North, Up) at a given anchor point.
pub fn enu_basis(anchor: &UvoxId) -> (Cartesian, Cartesian, Cartesian) {
    let anchor_cart = anchor.to_cartesian(crate::uvoxxyz::types::CoordSystem::Math);

    // Up = normalized anchor vector
    let up = normalize(anchor_cart);

    // Pick reference axis: use Z unless Up is too close, then use X
    let ref_axis = if up.z.abs() > 0.9 {
        Cartesian { x: 1.0, y: 0.0, z: 0.0 }
    } else {
        Cartesian { x: 0.0, y: 0.0, z: 1.0 }
    };

    // East = normalize(ref × Up)
    let east = normalize(cross(ref_axis, up));

    // North = normalize(Up × East)
    let north = normalize(cross(up, east));

    (east, north, up)
}

/// Transform global coordinates into local ENU relative to anchor.
pub fn to_local_enu(anchor: &UvoxId, point: &UvoxId) -> Cartesian {
    let anchor_cart = anchor.to_cartesian(crate::uvoxxyz::types::CoordSystem::Math);
    let point_cart = point.to_cartesian(crate::uvoxxyz::types::CoordSystem::Math);

    // Translate
    let dx = point_cart.x - anchor_cart.x;
    let dy = point_cart.y - anchor_cart.y;
    let dz = point_cart.z - anchor_cart.z;

    let (east, north, up) = enu_basis(anchor);

    Cartesian {
        x: dx * east.x + dy * east.y + dz * east.z,
        y: dx * north.x + dy * north.y + dz * north.z,
        z: dx * up.x + dy * up.y + dz * up.z,
    }
}

/// Convert local ENU coordinates back into global UvoxId.
pub fn from_local_enu(anchor: &UvoxId, local: Cartesian) -> UvoxId {
    let anchor_cart = anchor.to_cartesian(crate::uvoxxyz::types::CoordSystem::Math);
    let (east, north, up) = enu_basis(anchor);

    let global = Cartesian {
        x: anchor_cart.x + local.x * east.x + local.y * north.x + local.z * up.x,
        y: anchor_cart.y + local.x * east.y + local.y * north.y + local.z * up.y,
        z: anchor_cart.z + local.x * east.z + local.y * north.z + local.z * up.z,
    };

    UvoxId::from_cartesian(global, crate::uvoxxyz::types::CoordSystem::Math, anchor.frame_id)
}
