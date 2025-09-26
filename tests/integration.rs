use std::env;
use uuid::Uuid;

use omnivox::SimWorld;
use supabasic::Supabase;

#[tokio::test]
async fn test_load_simworld_from_supabase() {
    // Load .env so SUPABASE_URL and SUPABASE_KEY are available
    dotenvy::dotenv().ok();

    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY not set");
    let sup = Supabase::new(&url, &key);

    // Hardcoded simulation id from your setup
    let sim_id = Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")
        .expect("hardcoded UUID should parse");

    // Call into Omnivox (this does all the parsing)
    let world = SimWorld::load_from_supabase(&sup, sim_id)
        .await
        .expect("should load SimWorld from Supabase");

    println!("Loaded SimWorld: {:?}", world.simulation_id);
    assert_eq!(world.simulation_id, sim_id);
}
