use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CategoryId(pub u8);

/// High-level categories (u8)
pub static CATEGORY_MAP: Lazy<HashMap<CategoryId, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // 🧱 Structural / Elemental (raw or natural)
    m.insert(CategoryId(1),  "Metal");
    m.insert(CategoryId(2),  "Alloy");
    m.insert(CategoryId(3),  "Ceramic");
    m.insert(CategoryId(4),  "Glass");
    m.insert(CategoryId(5),  "Stone");
    m.insert(CategoryId(6),  "Concrete");
    m.insert(CategoryId(7),  "Brick");
    m.insert(CategoryId(8),  "Marble");
    m.insert(CategoryId(9),  "Granite");
    m.insert(CategoryId(10), "Wood");
    m.insert(CategoryId(11), "Bamboo");
    m.insert(CategoryId(12), "Cork");
    
    // 🧬 Synthetic / Processed
    m.insert(CategoryId(13), "Plastic");
    m.insert(CategoryId(14), "Rubber");
    m.insert(CategoryId(15), "Foam");
    m.insert(CategoryId(16), "Composite");
    m.insert(CategoryId(17), "Carbon Fiber");
    m.insert(CategoryId(18), "Fiberglass");
    m.insert(CategoryId(19), "Kevlar");
    m.insert(CategoryId(20), "Silicone");
    m.insert(CategoryId(21), "Asphalt");
    m.insert(CategoryId(22), "Paper");
    m.insert(CategoryId(23), "Textile");
    m.insert(CategoryId(24), "Leather");

    // ☁️ Other / Environmental
    m.insert(CategoryId(25), "Soil");
    m.insert(CategoryId(26), "Plasma");
    m.insert(CategoryId(27), "Water");

    m
});


impl CategoryId {
    pub fn name(self) -> &'static str {
        CATEGORY_MAP
            .get(&self)
            .copied()
            .unwrap_or("Unknown")
    }

    pub fn is_valid(self) -> bool {
        CATEGORY_MAP.contains_key(&self)
    }
}