use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::shared::app_state::AppState;

pub async fn init_session(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {

    // 1️⃣ Try to resume
    if let Some(sid) = headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        if let Ok(Some(session)) = app.session_source.resume(sid).await {
            return Json(json!({
                "status": "ok",
                "session_id": sid,
                "session": session,
                "reused": true
            }))
            .into_response();
        }
    }

    // 2️⃣ Create new anonymous session
    match app.session_source.create_anonymous().await {
        Ok((sid, mut session)) => {

            // 🔧 DEV MODE: immediately upgrade session to dev user
            if cfg!(debug_assertions) {
                // any email works with DevAuthSource
                if let Ok(user_id) = app
                    .auth_source
                    .login("dev@example.com".into(), "".into())
                    .await
                {
                    let _ = app
                        .session_source
                        .upgrade_to_user(sid, user_id)
                        .await;

                    // refresh session after upgrade
                    if let Ok(Some(updated)) = app.session_source.get_session(sid).await {
                        session = updated;
                    }
                }
            }

            Json(json!({
                "status": "ok",
                "session_id": sid,
                "session": session,
                "reused": false
            }))
            .into_response()
        }

        Err(e) => Json(json!({
            "error": e.to_string()
        }))
        .into_response(),
    }
}
