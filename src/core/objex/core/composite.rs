use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeProps {
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeObject {
    /// UUIDs of objects belonging to this composite
    pub children: Vec<String>,
}

impl CompositeObject {
    pub fn new(children: Vec<String>) -> Self {
        CompositeObject { children }
    }
}
