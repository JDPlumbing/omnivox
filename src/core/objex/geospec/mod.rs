pub mod primitives;
pub mod traits;
pub mod shape;
pub mod authoring;
pub mod inference;
pub mod api;
pub mod profile;

pub use primitives::*;
pub use traits::*;
pub use shape::*;
pub use authoring::*;
pub use inference::*;
pub use api::*;
pub use profile::*;
pub mod store;
pub use store::GeoSpecStore;

use serde::{Serialize, Deserialize};
//use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bounds {
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoSpec {
    pub bounds: Bounds,
    pub volume: f64,
    pub surface_area: f64,
}

impl Bounds {
    pub fn from_shape(shape: &AuthoringShape) -> Self {
        match shape {
            AuthoringShape::Point(_) => Bounds { radius: 0.0 },

            AuthoringShape::Line(line) => Bounds {
                radius: line.length / 2.0,
            },

            AuthoringShape::Plane(plane) => {
                let diag = (plane.width.powi(2) + plane.height.powi(2)).sqrt();
                Bounds { radius: diag / 2.0 }
            }

            AuthoringShape::Sphere(s) => Bounds { radius: s.radius },

            AuthoringShape::Box(b) => {
                let diag = (b.length.powi(2)
                    + b.width.powi(2)
                    + b.height.powi(2))
                    .sqrt();
                Bounds { radius: diag / 2.0 }
            }

            AuthoringShape::Cylinder(c) => {
                let diag = (4.0 * c.radius.powi(2) + c.height.powi(2)).sqrt();
                Bounds { radius: diag / 2.0 }
            }

            AuthoringShape::Cone(c) => {
                let diag = (4.0 * c.radius.powi(2) + c.height.powi(2)).sqrt();
                Bounds { radius: diag / 2.0 }
            }

            // ───────────── NEW: EXTRUDE ─────────────

            AuthoringShape::Extrude { profile, length, capped } => {
                use crate::core::objex::geospec::profile::Profile;

                let r_profile = match profile {
                    Profile::Circle { outer_radius, .. } => *outer_radius,

                    Profile::Rect { width, height, .. } => {
                        ((width / 2.0).powi(2) + (height / 2.0).powi(2)).sqrt()
                    }

                    Profile::IBeam {
                        flange_width,
                        web_height,
                        flange_thickness,
                        ..
                    } => {
                        let h = web_height + 2.0 * flange_thickness;
                        ((flange_width / 2.0).powi(2) + (h / 2.0).powi(2)).sqrt()
                    }
                };

                let radius = (r_profile.powi(2) + (length / 2.0).powi(2)).sqrt();

                Bounds { radius }
            }

            // ───────────── COMPOSITES ─────────────

            AuthoringShape::Union { children } => {
                let radius = children
                    .iter()
                    .map(|c| Bounds::from_shape(c).radius)
                    .fold(0.0, f64::max);

                Bounds { radius }
            }

            AuthoringShape::Difference { outer, .. } => {
                Bounds::from_shape(outer)
            }
        }
    }
}


