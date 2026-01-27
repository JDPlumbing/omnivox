use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateTimeEntity {
    pub sim_time: i128,
}
