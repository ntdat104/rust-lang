use axum::{
    extract::{ Path, State },
    http::StatusCode,
    response::{ IntoResponse, Json },
    routing::{ get },
    Router,
};
use serde_json::json;
use std::sync::{ Arc, Mutex };
use uuid::Uuid;
use utoipa::OpenApi;

use crate::models::User;

type Db = Arc<Mutex<Vec<User>>>;

#[derive(OpenApi)]
#[openapi(
    paths(get_users, create_user, get_user, update_user, delete_user),
    components(schemas(User))
)]
pub struct ApiDoc;

pub fn app_router(db: Db) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(db)
}

#[utoipa::path(
    get,
    path = "/users",
    responses((status = 200, description = "Get all users", body = [User]))
)]
async fn get_users(State(db): State<Db>) -> impl IntoResponse {
    let users = db.lock().unwrap();
    (StatusCode::OK, Json(users.clone()))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses((status = 201, description = "User created", body = User))
)]
async fn create_user(State(db): State<Db>, Json(payload): Json<User>) -> impl IntoResponse {
    let mut users = db.lock().unwrap();
    let mut user = payload.clone();
    user.id = Uuid::new_v4().to_string();
    users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(Path(id): Path<String>, State(db): State<Db>) -> impl IntoResponse {
    let users = db.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == id) {
        (StatusCode::OK, Json(json!(user))).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    request_body = User,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn update_user(
    Path(id): Path<String>,
    State(db): State<Db>,
    Json(payload): Json<User>
) -> impl IntoResponse {
    let mut users = db.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.name = payload.name;
        user.email = payload.email;
        (StatusCode::OK, Json(user.clone())).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
    }
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found")
    )
)]
async fn delete_user(Path(id): Path<String>, State(db): State<Db>) -> impl IntoResponse {
    let mut users = db.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == id) {
        users.remove(pos);
        (StatusCode::OK, Json(json!({ "message": "User deleted" }))).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
    }
}
