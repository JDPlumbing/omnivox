use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Variants (category_id, variant_id)
pub static VARIANT_MAP: Lazy<HashMap<(u8, u16), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // 1 - Metal
    m.insert((1, 1), "Steel");
    m.insert((1, 2), "Aluminum");
    m.insert((1, 3), "Copper");
    m.insert((1, 4), "Iron");
    m.insert((1, 5), "Titanium");

    // 2 - Plastic
    m.insert((2, 1), "PVC");
    m.insert((2, 2), "Polyethylene");
    m.insert((2, 3), "Polycarbonate");
    m.insert((2, 4), "Nylon");
    m.insert((2, 5), "PTFE");

    // 3 - Wood
    m.insert((3, 1), "Pine");
    m.insert((3, 2), "Oak");
    m.insert((3, 3), "Birch");
    m.insert((3, 4), "Mahogany");

    // 4 - Composite
    m.insert((4, 1), "Carbon Fiber Reinforced Polymer");
    m.insert((4, 2), "Glass Fiber Reinforced Polymer");
    m.insert((4, 3), "Kevlar Composite");

    // 5 - Glass
    m.insert((5, 1), "Soda-Lime");
    m.insert((5, 2), "Borosilicate");
    m.insert((5, 3), "Tempered");

    // 6 - Ceramic
    m.insert((6, 1), "Porcelain");
    m.insert((6, 2), "Alumina");
    m.insert((6, 3), "Silicon Carbide");

    // 7 - Textile
    m.insert((7, 1), "Cotton");
    m.insert((7, 2), "Polyester");
    m.insert((7, 3), "Wool");
    m.insert((7, 4), "Nylon Fiber");

    // 8 - Rubber
    m.insert((8, 1), "Natural Rubber");
    m.insert((8, 2), "Neoprene");
    m.insert((8, 3), "Silicone Rubber");

    // 9 - Concrete
    m.insert((9, 1), "Standard Mix");
    m.insert((9, 2), "High Strength");
    m.insert((9, 3), "Lightweight");

    // 10 - Stone
    m.insert((10, 1), "Limestone");
    m.insert((10, 2), "Sandstone");
    m.insert((10, 3), "Basalt");

    // 11 - Paper
    m.insert((11, 1), "Cardboard");
    m.insert((11, 2), "Printer Paper");
    m.insert((11, 3), "Recycled Pulp");

    // 12 - Leather
    m.insert((12, 1), "Cowhide");
    m.insert((12, 2), "Suede");
    m.insert((12, 3), "Synthetic Leather");

    // 13 - Foam
    m.insert((13, 1), "Polyurethane Foam");
    m.insert((13, 2), "Polystyrene Foam");
    m.insert((13, 3), "Memory Foam");

    // 14 - Asphalt
    m.insert((14, 1), "Hot Mix Asphalt");
    m.insert((14, 2), "Cold Mix Asphalt");

    // 15 - Brick
    m.insert((15, 1), "Clay Brick");
    m.insert((15, 2), "Fly Ash Brick");
    m.insert((15, 3), "Concrete Brick");

    // 16 - Bamboo
    m.insert((16, 1), "Natural Bamboo");
    m.insert((16, 2), "Laminated Bamboo");

    // 17 - Cork
    m.insert((17, 1), "Natural Cork");
    m.insert((17, 2), "Agglomerated Cork");

    // 18 - Carbon Fiber
    m.insert((18, 1), "Standard Modulus");
    m.insert((18, 2), "High Modulus");

    // 19 - Kevlar
    m.insert((19, 1), "Kevlar 29");
    m.insert((19, 2), "Kevlar 49");

    // 20 - Alloy
    m.insert((20, 1), "Brass");
    m.insert((20, 2), "Bronze");
    m.insert((20, 3), "Stainless Steel");

    // 21 - Fiberglass
    m.insert((21, 1), "E-Glass");
    m.insert((21, 2), "S-Glass");

    // 22 - Silicone
    m.insert((22, 1), "High-Temp Silicone");
    m.insert((22, 2), "Medical Grade Silicone");

    // 23 - Gypsum
    m.insert((23, 1), "Plaster");
    m.insert((23, 2), "Drywall Core");

    // 24 - Marble
    m.insert((24, 1), "White Marble");
    m.insert((24, 2), "Black Marble");

    // 25 - Granite
    m.insert((25, 1), "Gray Granite");
    m.insert((25, 2), "Red Granite");

    m
});
