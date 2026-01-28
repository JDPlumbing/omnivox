use anyhow::Result;
use crate::core::UserId;
use crate::shared::identity::IdentitySource;
use crate::shared::identity::auth_context::AccountRole;

pub async fn resolve_identity(
    identity_source: &dyn IdentitySource,
    external_id: &str,
) -> Result<(UserId, AccountRole)> {
    if let Some((user_id, role)) =
        identity_source.lookup_by_external_id(external_id).await?
    {
        return Ok((user_id, role));
    }

    let user_id = UserId::new();
    let role = AccountRole::User;

    identity_source
        .create_mapping(external_id, &user_id, role)
        .await?;

    Ok((user_id, role))
}
