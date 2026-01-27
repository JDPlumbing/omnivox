use axum::{
    extract::State,
    Json,
};
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::engine::entities::entity_engine::EntityEngine;
use crate::core::components::note::Note;
use crate::core::EntityId;

use crate::api::entities::dtos::create_note::CreateNoteDto;

pub async fn create_note_entity(
    State(app): State<AppState>,
    Json(payload): Json<CreateNoteDto>,
) -> Json<serde_json::Value> {
    let mut store = app.entity_store.write().await;
    let mut engine = EntityEngine::new(&mut store);

    let note = Note {
        text: payload.text,
    };

    let entity_id = match payload.entity_id {
        Some(id) => {
            let eid = EntityId(id);
            engine.add_note(eid, note);
            eid
        }
        None => engine.create_note_entity(note),
    };

    Json(serde_json::json!({
        "entity_id": entity_id.to_string()
    }))
}
