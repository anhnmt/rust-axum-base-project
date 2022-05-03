use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn find_all_users() -> impl IntoResponse {
    Json(json!({"a":"b"}))
}

pub async fn find_user_by_id() -> impl IntoResponse {
    Json(json!({"a":"b"}))
}