use axum::{
    Router,
    extract::{Path, Query},
    routing,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page_id: Option<String>,
}

pub fn routes() -> Router {
    let router = Router::new()
        .route("/", routing::get(list))
        .route("/{category_id}", routing::get(find_one));
    return router;
}

async fn list(Query(pagination): Query<Pagination>) -> String {
    let page_id = pagination.page_id.unwrap_or_default();

    String::from("Page ID: ") + &page_id
}

async fn find_one(Path(category_id): Path<String>) -> String {
    String::from("Category ID: ") + &category_id
}
