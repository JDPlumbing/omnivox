/* use uuid::Uuid;
use supabasic::Supabase;
use objex::{Objex, Shape, MaterialLink};
use chronovox::UvoxId;

use omnivox::sim::persist::spawn_entity_with_objex;

#[tokio::test]
async fn test_spawn_entity_with_objex() {
    dotenvy::dotenv().ok();
    let url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let key = std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY not set");
    let sup = Supabase::new(&url, &key);

    let sim_id = Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")
        .expect("hardcoded sim id");

    let obj = Objex {
        entity_id: Uuid::new_v4(),
        name: "PersistTest House".into(),
        shape: Shape {
            geometry: serde_json::json!({
                "type": "cube",
                "width": 10.0,
                "height": 8.0,
                "depth": 12.0
            }),
        },
        material: MaterialLink {
            category_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccc0001").unwrap(),
            properties: serde_json::json!({"composition": "wood"}),
        },
    };

    let uvox = UvoxId::earth(0, 0, 0);

    let result = spawn_entity_with_objex(&sup, sim_id, 0, obj, uvox).await;
    assert!(result.is_ok(), "spawn_entity_with_objex failed: {:?}", result);
}
*/