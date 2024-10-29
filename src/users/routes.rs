use super::UserHandlers;
use crate::AppState;

use axum::{routing::get, Router};
pub struct UserRoutes;

impl UserRoutes {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/users", get(UserHandlers::list).post(UserHandlers::create))
            .route(
                "/users/:id",
                get(UserHandlers::get)
                    .put(UserHandlers::update)
                    .delete(UserHandlers::delete),
            )
    }
}
