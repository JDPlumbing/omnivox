use crate::supabasic::properties::PropertyRecord;
use crate::core::objex::core::{Objex, MaterialLink, MaterialName};
use crate::core::objex::Shape;
use crate::core::objex::matcat::materials::MaterialName;
use crate::core::objex::geospec::shapes::BoxShape;
use uuid::Uuid;

pub fn generate_walls(property: &PropertyRecord) -> Vec<Objex> {
    let mut walls = Vec::new();

    // 1️⃣ Pull from property values
    let sqft = property.square_feet.unwrap_or(1200) as f64;
    let wall_height = property.ceiling_height_ft.unwrap_or(8.0);
    let side_length = sqft.sqrt();
    let perimeter = 4.0 * side_length;

    // 2️⃣ Wall material and block geometry
    let block_length = 1.333; // 16"
    let block_height = 0.667; // 8"
    let block_depth = 0.667; // 8"
    let num_blocks_per_side = (side_length / block_length).ceil() as usize;
    let num_rows = (wall_height / block_height).ceil() as usize;
    let block_material = MaterialLink::new(MaterialName::Concrete);

    // 3️⃣ Generate outer wall blocks
    for side in 0..4 {
        for row in 0..num_rows {
            for i in 0..num_blocks_per_side {
                let name = format!("Wall block side{} row{} unit{}", side, row, i);

                let wall_block = Objex::new(
                    property.frame_id.unwrap_or(0),
                    property.property_id,
                    name,
                    Shape::Box(BoxShape {
                        length: block_length,
                        width: block_depth,
                        height: block_height,
                    }),
                    block_material.clone(),
                )
                .with_metadata("trade", "framing")
                .with_metadata("category", "wall_block")
                .with_metadata("structural", "true");

                walls.push(wall_block);
            }
        }
    }

    walls
}
