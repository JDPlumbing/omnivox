use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use supabasic::Supabase;
use omnivox::sim::SimWorld;
use omnivox::sim::error::OmnivoxError;

#[tokio::test]
async fn test_load_simworld_from_supabase() -> Result<(), OmnivoxError> {
    dotenv().ok();

    let url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY not set");
    let sup = Supabase::new(&url, &key);

    // Hardcoded test simulation UUID
    let sim_id = Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")
        .expect("valid UUID");

    let world = SimWorld::load_from_supabase(&sup, sim_id).await?;

    // Assert metadata was loaded
    assert_eq!(world.simulation_id, sim_id);
    assert_eq!(world.frame_id, 0); // default test frame
    assert_eq!(world.owner_id.to_string(), "4ea96b3f-51d7-4238-bd18-2f7fd8be26ec");

    // For now, objects/timeline are still empty
    assert!(world.objects.is_empty());
    assert!(world.timeline.is_empty());

    Ok(())
}
