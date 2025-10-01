use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use crate::supabasic::client::Supabase;
use crate::supabasic::error::SupabasicError;

/// Trait every table struct should implement
pub trait DbModel: Sized + DeserializeOwned + Serialize {
    fn table() -> &'static str;
}

// === Generic CRUD === //

/// Fetch a single row by UUID
pub async fn fetch<T: DbModel>(id: Uuid) -> Result<T, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(T::table())
        .select("*")
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// List all rows in the table
pub async fn list<T: DbModel>() -> Result<Vec<T>, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(T::table())
        .select("*")
        .execute()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// Insert a new row and return the inserted row
pub async fn insert<I, O>(item: &I) -> Result<O, SupabasicError>
where
    I: Serialize,
    O: DbModel + DeserializeOwned,
{
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(O::table())
        .insert(serde_json::to_value(item)?)
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// Update a row by UUID (returns the updated row)
pub async fn update<I, O>(id: Uuid, item: &I) -> Result<O, SupabasicError>
where
    I: Serialize,
    O: DbModel + DeserializeOwned,
{
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(O::table())
        .update(serde_json::to_value(item)?)
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}

/// Delete a row by UUID (returns the deleted row, if found)
pub async fn delete<T: DbModel>(id: Uuid) -> Result<T, SupabasicError> {
    let supa = Supabase::new_from_env()?;
    let val = supa
        .from(T::table())
        .delete()
        .eq("id", &id.to_string())
        .single()
        .await?;
    Ok(serde_json::from_value(val)?)
}
