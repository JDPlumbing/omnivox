// shared/permissions/engine.rs
use crate::shared::ownership::ownership_source::OwnershipContext;
use crate::shared::identity::auth_context::AccountRole;
use crate::core::{UserId, WorldId};

#[derive(Debug, Clone)]
pub enum Action {
    View,
    Edit,
    Delete,
    Simulate,
    Admin,
}

#[derive(Debug, Clone)]
pub enum Resource {
    World(WorldId),
    Property,
    User(UserId),
}

pub fn can(
    ownership: &OwnershipContext,
    role: AccountRole,
    action: Action,
    resource: Resource,
) -> bool {
    match action {
        Action::View => true,

        Action::Edit => {
            ownership.property_role.as_deref() == Some("owner")
        }

        Action::Simulate => {
            ownership.world_id.is_some()
        }

        Action::Admin => role == AccountRole::Root,

        _ => false,
    }
}
