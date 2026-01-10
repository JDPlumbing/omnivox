use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

use crate::shared::app_state::AppState;

use crate::core::objex::core::types::Objex;
use crate::core::objex::matcat::materials::MatCatId;
use crate::core::objex::geospec::GeoSpec;


/// ─────────────────────────────────────────────
/// Internal API DTOs
/// (NOT HTTP, NOT DB)
/// ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateObjex {
    pub geospec_id: Uuid,
    pub matcat: MatCatId,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjexView {
    pub id: Uuid,
    pub geospec_id: Uuid,
    pub matcat: MatCatId,
    pub metadata: Value,
}


/// ─────────────────────────────────────────────
/// Internal API functions
/// ─────────────────────────────────────────────

pub async fn list_objex_templates(
    state: &AppState,
) -> anyhow::Result<Vec<ObjexView>> {
    let store = state.objex_store.read().await;

    let out = store
        .list()
        .into_iter()
        .map(|objex| ObjexView {
            id: objex.id,
            geospec_id: objex.geospec_id,
            matcat: objex.matcat,
            metadata: Value::Object(Default::default()),
        })
        .collect();

    Ok(out)
}


pub async fn create_objex_template(
    state: &AppState,
    req: CreateObjex,
) -> anyhow::Result<ObjexView> {
    let mut store = state.objex_store.write().await;

    let objex = Objex {
        id: Uuid::new_v4(),
        geospec_id: req.geospec_id,
        matcat: req.matcat,
    };

    store.insert(objex.clone());

    Ok(ObjexView {
        id: objex.id,
        geospec_id: objex.geospec_id,
        matcat: objex.matcat,
        metadata: req.metadata.unwrap_or_else(|| Value::Object(Default::default())),
    })
}
