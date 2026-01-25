use uuid::Uuid;
use crate::core::UserId;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub supabase_user_id: Uuid,
    pub user_id: UserId,
    pub account_role: AccountRole,
}

#[derive(Clone, Debug)]
pub enum AccountRole {
    Root,
    User,
}
