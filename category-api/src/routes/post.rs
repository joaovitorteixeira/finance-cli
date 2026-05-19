use axum::{extract::Json, Router, routing};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateCategory {
    name: String
}

pub fn routes() -> Router {
    let router = Router::new().route("/", routing::post(create));
    return router;
}

async fn create(Json(payload): Json<CreateCategory>) -> String {
    String::from("Category Name: ") + &payload.name
}
