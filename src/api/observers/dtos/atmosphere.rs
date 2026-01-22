use serde::Serialize;

#[derive(Serialize)]
pub struct AtmosphereOpticsResponse {
    pub observer_id: u64,
    pub time_ns: i128,

    pub optical_depth: f64,
    pub transmittance: f64,

    /// Integrated scattering energy (physics)
    pub sky_scatter_energy: f64,

    /// Cosine of sun elevation (0–1)
    pub sun_visibility: f64,

    /// Perceptual-ish sky brightness
    pub sky_luminance: f64,
}
