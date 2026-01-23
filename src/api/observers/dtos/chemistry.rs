use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct AtmosphereChemistryResponse {
    pub observer_id: u64,
    pub time_ns: i128,

    /// Partial pressures per species (Pa)
    pub partial_pressure_pa: HashMap<String, f64>,

    /// Mass density per species (kg/m³)
    pub mass_density_kg_m3: HashMap<String, f64>,
}
