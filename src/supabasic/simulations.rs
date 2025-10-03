use crate::supabasic::orm::DbModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRow {
    pub simulation_id: Uuid,
    pub owner_id: Uuid,
    pub tick_rate: i64,
    pub last_saved: Option<DateTime<Utc>>,
    pub frame_id: i64,
}

impl DbModel for SimulationRow {
    fn table() -> &'static str { "simulations" }
}
