use std::collections::HashMap;
use once_cell::sync::Lazy;

/// High-level categories (u8)
pub static CATEGORY_MAP: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // 🧱 Structural / Elemental (raw or natural)
    m.insert(1,  "Metal");
    m.insert(2,  "Alloy");
    m.insert(3,  "Ceramic");
    m.insert(4,  "Glass");
    m.insert(5,  "Stone");
    m.insert(6,  "Concrete");
    m.insert(7,  "Brick");
    m.insert(8,  "Marble");
    m.insert(9,  "Granite");
    m.insert(10, "Wood");
    m.insert(11, "Bamboo");
    m.insert(12, "Cork");

    // 🧬 Synthetic / Processed
    m.insert(13, "Plastic");
    m.insert(14, "Rubber");
    m.insert(15, "Foam");
    m.insert(16, "Composite");
    m.insert(17, "Carbon Fiber");
    m.insert(18, "Fiberglass");
    m.insert(19, "Kevlar");
    m.insert(20, "Silicone");
    m.insert(21, "Asphalt");
    m.insert(22, "Paper");
    m.insert(23, "Textile");
    m.insert(24, "Leather");

    // ☁️ Other / Environmental
    m.insert(25, "Soil");
    m.insert(26, "Plasma");
    m.insert(27, "Water");

    m
});