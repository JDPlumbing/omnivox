use crate::core::env::FieldSample;


#[derive(Debug, Clone)]
pub struct OceanChemistrySample {
    pub salinity_psu: f64,
    pub dissolved_oxygen_kg_m3: f64,
    pub dissolved_co2_kg_m3: f64,
    pub density_kg_m3: f64,
}
pub struct OceanChemistry {
    pub salinity_psu: f64,
    pub dissolved_oxygen_kg_m3: f64,
    pub dissolved_co2_kg_m3: f64,
}
impl OceanChemistry {
    pub fn earth_like() -> Self {
        Self {
            salinity_psu: 35.0,
            dissolved_oxygen_kg_m3: 0.008, // ~8 mg/L
            dissolved_co2_kg_m3: 0.002,
        }
    }
}
impl OceanChemistry {
    pub fn sample(&self, _env: &FieldSample) -> OceanChemistrySample {
        // Simple linear density model for now
        let density = 1000.0 + self.salinity_psu * 0.7;

        OceanChemistrySample {
            salinity_psu: self.salinity_psu,
            dissolved_oxygen_kg_m3: self.dissolved_oxygen_kg_m3,
            dissolved_co2_kg_m3: self.dissolved_co2_kg_m3,
            density_kg_m3: density,
        }
    }
}
