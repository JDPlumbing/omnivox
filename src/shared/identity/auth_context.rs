use crate::core::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user_id: UserId,
    pub role: AccountRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountRole {
    Root,
    User,
}
