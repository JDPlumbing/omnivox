use crate::sim::world::SimWorld;
use crate::sim::error::Result;
use supabasic::Supabase;

impl SimWorld {
    pub async fn save_to_supabase(&self, _sup: &Supabase) -> Result<()> {
        // TODO: implement persistence
        Ok(())
    }
}
