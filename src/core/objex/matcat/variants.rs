use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::core::objex::matcat::categories::{CategoryId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VariantId(pub u16);

/// Variants (category_id, variant_id)
pub static VARIANT_MAP: Lazy<HashMap<(CategoryId, VariantId), &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // 1 - Metal
    m.insert((CategoryId(1), VariantId(1)), "Steel");
    m.insert((CategoryId(1), VariantId(2)), "Aluminum");
    m.insert((CategoryId(1), VariantId(3)), "Copper");
    m.insert((CategoryId(1), VariantId(4)), "Iron");
    m.insert((CategoryId(1), VariantId(5)), "Titanium");

    // 2 - Plastic
    m.insert((CategoryId(2), VariantId(1)), "PVC");
    m.insert((CategoryId(2), VariantId(2)), "Polyethylene");
    m.insert((CategoryId(2), VariantId(3)), "Polycarbonate");
    m.insert((CategoryId(2), VariantId(4)), "Nylon");
    m.insert((CategoryId(2), VariantId(5)), "PTFE");

    // 3 - Wood
    m.insert((CategoryId(3), VariantId(1)), "Pine");
    m.insert((CategoryId(3), VariantId(2)), "Oak");
    m.insert((CategoryId(3), VariantId(3)), "Birch");
    m.insert((CategoryId(3), VariantId(4)), "Mahogany");

    // 4 - Composite
    m.insert((CategoryId(4), VariantId(1)), "Carbon Fiber Reinforced Polymer");
    m.insert((CategoryId(4), VariantId(2)), "Glass Fiber Reinforced Polymer");
    m.insert((CategoryId(4), VariantId(3)), "Kevlar Composite");

    // 5 - Glass
    m.insert((CategoryId(5), VariantId(1)), "Soda-Lime");
    m.insert((CategoryId(5), VariantId(2)), "Borosilicate");
    m.insert((CategoryId(5), VariantId(3)), "Tempered");

    // 6 - Ceramic
    m.insert((CategoryId(6), VariantId(1)), "Porcelain");
    m.insert((CategoryId(6), VariantId(2)), "Alumina");
    m.insert((CategoryId(6), VariantId(3)), "Silicon Carbide");

    // 7 - Textile
    m.insert((CategoryId(7), VariantId(1)), "Cotton");
    m.insert((CategoryId(7), VariantId(2)), "Polyester");
    m.insert((CategoryId(7), VariantId(3)), "Wool");
    m.insert((CategoryId(7), VariantId(4)), "Nylon Fiber");

    // 8 - Rubber
    m.insert((CategoryId(8), VariantId(1)), "Natural Rubber");
    m.insert((CategoryId(8), VariantId(2)), "Neoprene");
    m.insert((CategoryId(8), VariantId(3)), "Silicone Rubber");

    // 9 - Concrete
    m.insert((CategoryId(9), VariantId(1)), "Standard Mix");
    m.insert((CategoryId(9), VariantId(2)), "High Strength");
    m.insert((CategoryId(9), VariantId(3)), "Lightweight");

    // 10 - Stone
    m.insert((CategoryId(10), VariantId(1)), "Limestone");
    m.insert((CategoryId(10), VariantId(2)), "Sandstone");
    m.insert((CategoryId(10), VariantId(3)), "Basalt");

    // 11 - Paper
    m.insert((CategoryId(11), VariantId(1)), "Cardboard");
    m.insert((CategoryId(11), VariantId(2)), "Printer Paper");
    m.insert((CategoryId(11), VariantId(3)), "Recycled Pulp");

    // 12 - Leather
    m.insert((CategoryId(12), VariantId(1)), "Cowhide");
    m.insert((CategoryId(12), VariantId(2)), "Suede");
    m.insert((CategoryId(12), VariantId(3)), "Synthetic Leather");

    // 13 - Foam
    m.insert((CategoryId(13), VariantId(1)), "Polyurethane Foam");
    m.insert((CategoryId(13), VariantId(2)), "Polystyrene Foam");
    m.insert((CategoryId(13), VariantId(3)), "Memory Foam");

    // 14 - Asphalt
    m.insert((CategoryId(14), VariantId(1)), "Hot Mix Asphalt");
    m.insert((CategoryId(14), VariantId(2)), "Cold Mix Asphalt");
    
    // 15 - Brick
    m.insert((CategoryId(15), VariantId(1)), "Clay Brick");
    m.insert((CategoryId(15), VariantId(2)), "Fly Ash Brick");
    m.insert((CategoryId(15), VariantId(3)), "Concrete Brick");
    
    // 16 - Bamboo
    m.insert((CategoryId(16), VariantId(1)), "Natural Bamboo");
    m.insert((CategoryId(16), VariantId(2)), "Laminated Bamboo");

    // 17 - Cork
    m.insert((CategoryId(17), VariantId(1)), "Natural Cork");
    m.insert((CategoryId(17), VariantId(2)), "Agglomerated Cork");
    
    // 18 - Carbon Fiber
    m.insert((CategoryId(18), VariantId(1)), "Standard Modulus");
    m.insert((CategoryId(18), VariantId(2)), "High Modulus");
    m.insert((CategoryId(18), VariantId(3)), "Ultra-High Modulus");

    // 19 - Kevlar
    m.insert((CategoryId(19), VariantId(1)), "Kevlar 29");
    m.insert((CategoryId(19), VariantId(2)), "Kevlar 49");
    
    // 20 - Alloy
    m.insert((CategoryId(20), VariantId(1)), "Brass");
    m.insert((CategoryId(20), VariantId(2)), "Bronze");
    m.insert((CategoryId(20), VariantId(3)), "Stainless Steel");
    
    // 21 - Fiberglass
    m.insert((CategoryId(21), VariantId(1)), "E-Glass");
    m.insert((CategoryId(21), VariantId(2)), "S-Glass");

    // 22 - Silicone
    m.insert((CategoryId(22), VariantId(1)), "High-Temp Silicone");
    m.insert((CategoryId(22), VariantId(2)), "Medical Grade Silicone");
    
    // 23 - Gypsum
    m.insert((CategoryId(23), VariantId(1)), "Plaster");
    m.insert((CategoryId(23), VariantId(2)), "Drywall Core");

    // 24 - Marble
    m.insert((CategoryId(24), VariantId(1)), "White Marble");
    m.insert((CategoryId(24), VariantId(2)), "Black Marble");
    
    // 25 - Granite
    m.insert((CategoryId(25), VariantId(1)), "Gray Granite");
    m.insert((CategoryId(25), VariantId(2)), "Red Granite");

    // 26 - Plasma
    m.insert((CategoryId(26), VariantId(1)), "Stellar Plasma");
    
    // 27 - Water
    m.insert((CategoryId(27), VariantId(1)), "Fresh Water");
    m.insert((CategoryId(27), VariantId(2)), "Salt Water");

    m
});


impl VariantId {
    pub fn name(self, category: CategoryId) -> &'static str {
        VARIANT_MAP
            .get(&(category, self))
            .copied()
            .unwrap_or("Generic")
    }
}
