use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub text: String,
}
impl From<&str> for Note {
    fn from(s: &str) -> Self {
        Note {
            text: s.to_string(),
        }
    }
}
