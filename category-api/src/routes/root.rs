use axum::{Router, routing::IntoMakeService};

use crate::routes::{delete, get, post};

pub fn routes() -> IntoMakeService<Router> {
    let merged_router = { get::routes().merge(post::routes()).merge(delete::routes()) };

    let app_router = Router::new()
        .nest("/category", merged_router);

    app_router.into_make_service()
}
