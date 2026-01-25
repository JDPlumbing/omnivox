use uuid::Uuid;
use crate::core::UserId;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub session_id: Option<Uuid>,
    pub user_id: Option<UserId>,
    pub owner_id: Option<UserId>,
    pub is_anon: bool,
}

impl RequestContext {
    pub fn anonymous(session_id: Option<Uuid>) -> Self {
        Self {
            session_id,
            user_id: None,
            owner_id: None,
            is_anon: true,
        }
    }

    pub fn authenticated(
        session_id: Option<Uuid>,
        user_id: UserId,
    ) -> Self {
        Self {
            session_id,
            user_id: Some(user_id),
            owner_id: Some(user_id),
            is_anon: false,
        }
    }
}
