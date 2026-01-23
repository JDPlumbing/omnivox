use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OceanChemistryResponse {
    pub observer_id: u64,
    pub time_ns: i128,

    pub salinity_psu: f64,
    pub dissolved_oxygen_kg_m3: f64,
    pub dissolved_co2_kg_m3: f64,
    pub density_kg_m3: f64,
}
