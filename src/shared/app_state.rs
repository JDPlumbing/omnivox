use uuid::Uuid;
use crate::supabasic::Supabase;

#[derive(Clone)]
pub struct AppState {
    pub supa: Supabase,
    pub session_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
}

impl AppState {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let supa = Supabase::new_from_env()?;

        Ok(Self {
            supa,
            session_id: None,
            user_owner_id: None,
            anon_owner_id: None,
        })
    }

    /// Convenience method for attaching a session or user
    pub fn with_session(mut self, session_id: Uuid, anon_owner_id: Option<Uuid>) -> Self {
        self.session_id = Some(session_id);
        self.anon_owner_id = anon_owner_id;
        self
    }
}
