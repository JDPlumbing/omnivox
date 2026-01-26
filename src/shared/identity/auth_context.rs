use crate::core::UserId;

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub user_id: UserId,
    pub role: AccountRole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountRole {
    Root,
    User,
}
