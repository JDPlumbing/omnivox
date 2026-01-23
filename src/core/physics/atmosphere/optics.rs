use crate::core::env::atmosphere::AtmosphereField;
//use crate::core::env::fields::Field;
//use crate::core::uvoxid::UvoxId;
//use crate::core::tdt::SimDuration;
use crate::core::math::vec3::{normalize, magnitude};

/// Result of integrating atmosphere along a ray
#[derive(Debug, Clone, Copy)]
pub struct AtmosphereOpticsResult {
    /// Integrated optical depth (unitless)
    pub optical_depth: f64,

    /// exp(-optical_depth)
    pub transmittance: f64,

    /// Rough heuristic brightness of scattered sky light
    pub sky_scatter_energy: f64,
}

/// Parameters controlling sampling quality
#[derive(Debug, Clone, Copy)]
pub struct AtmosphereOpticsParams {
    /// Maximum distance to march (meters)
    pub max_distance_m: f64,

    /// Step size along ray (meters)
    pub step_m: f64,

    /// Extinction coefficient (how strongly air blocks light)
    pub extinction_coeff: f64,

    /// Scattering coefficient (how much light contributes to sky glow)
    pub scattering_coeff: f64,
}

impl Default for AtmosphereOpticsParams {
    fn default() -> Self {
        Self {
            max_distance_m: 120_000.0,
            step_m: 500.0,
            extinction_coeff: 1.0e-5,   // ✅ key fix
            scattering_coeff: 0.1,
        }
    }
}


/// Integrate atmospheric effects along a ray
///
/// `origin` and `dir` are world-space.
/// `dir` does not need to be normalized.
pub fn integrate_atmosphere_along_ray(
    atmosphere: &AtmosphereField,
    origin: [f64; 3],
    view_dir: [f64; 3],
    sun_dir: [f64; 3],
    params: AtmosphereOpticsParams,
) -> AtmosphereOpticsResult {
    let view_dir = normalize(view_dir);
    let sun_dir = normalize(sun_dir);

    let mut optical_depth = 0.0;
    let mut scattered_light = 0.0;
    let mut t = 0.0;

    while t < params.max_distance_m {
        let p = [
            origin[0] + view_dir[0] * t,
            origin[1] + view_dir[1] * t,
            origin[2] + view_dir[2] * t,
        ];

        let r = magnitude(p);
        let density = atmosphere.density_at_radius(r);

        // --- extinction along view ray ---
        optical_depth += density * params.extinction_coeff * params.step_m;

        // --- sun-angle scattering ---
        let sun_zenith_cos = sun_dir[2].clamp(0.05, 1.0); // assume +Z = up
        let air_mass = 1.0 / sun_zenith_cos;

        let local_scatter =
            density * params.scattering_coeff * air_mass;

        scattered_light += local_scatter * params.step_m;

        t += params.step_m;
    }

    let transmittance = (-optical_depth).exp();
    let sky_scatter_energy = scattered_light * transmittance;

    AtmosphereOpticsResult {
        optical_depth,
        transmittance,
        sky_scatter_energy,
    }
}

