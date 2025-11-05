use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Grade tiers for each (category, variant) combination.
/// Represented by the last 2 bytes of MatCatId.
pub static GRADE_MAP: Lazy<HashMap<(u8, u16, u16), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // Metals - Steel
    m.insert((1, 1, 1), "A36 Structural");
    m.insert((1, 1, 2), "304 Stainless");
    m.insert((1, 1, 3), "4140 Alloy");
    m.insert((1, 1, 4), "Tool Steel");

    // Metals - Aluminum
    m.insert((1, 2, 1), "6061-T6");
    m.insert((1, 2, 2), "7075-T6");
    m.insert((1, 2, 3), "1100 Pure");

    // Metals - Copper
    m.insert((1, 3, 1), "C110 Electrolytic Tough Pitch");
    m.insert((1, 3, 2), "C122 Phosphor Deoxidized");

    // Concrete
    m.insert((9, 1, 1), "M20");
    m.insert((9, 1, 2), "M25");
    m.insert((9, 1, 3), "M40");

    // Wood
    m.insert((3, 1, 1), "Construction Grade");
    m.insert((3, 1, 2), "Select Structural");
    m.insert((3, 1, 3), "Cabinet Grade");

    // Plastics - PVC
    m.insert((2, 1, 1), "Schedule 40");
    m.insert((2, 1, 2), "Schedule 80");

    // Composites
    m.insert((4, 1, 1), "Standard Modulus");
    m.insert((4, 1, 2), "High Modulus");
    m.insert((4, 1, 3), "Ultra-High Modulus");

    // Glass
    m.insert((5, 1, 1), "Annealed");
    m.insert((5, 1, 2), "Tempered");
    m.insert((5, 1, 3), "Laminated");

    // Ceramics
    m.insert((6, 1, 1), "Low-Fire");
    m.insert((6, 1, 2), "Mid-Fire");
    m.insert((6, 1, 3), "High-Fire");

    // Rubbers
    m.insert((8, 1, 1), "Soft");
    m.insert((8, 1, 2), "Medium");
    m.insert((8, 1, 3), "Hard");

    // General fallback tiers for any category/variant
    for cat in 1..=25 {
        for var in 1..=5 {
            m.entry((cat, var, 99)).or_insert("Standard Grade");
            m.entry((cat, var, 100)).or_insert("High Grade");
        }
    }

    m
});
