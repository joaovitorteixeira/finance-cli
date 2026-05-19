use axum::{Router, extract::Path, routing};

pub fn routes() -> Router {
    let router = Router::new().route("/{category_id}", routing::delete(delete));

    return router;
}

async fn delete(Path(category_id): Path<String>) -> String {
    String::from("Category ID: ") + &category_id
}
