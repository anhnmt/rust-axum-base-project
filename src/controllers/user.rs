use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde_json::json;

use crate::models::user::{CreateUser, User};

pub async fn find_all_users() -> impl IntoResponse {
    Json(json!({"a":"b"}))
}

pub async fn find_user_by_id() -> impl IntoResponse {
    Json(json!({"a":"b"}))
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::OK, Json(user))
}