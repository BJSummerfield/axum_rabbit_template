use super::UserHandlers;
use crate::AppState;

use axum::{routing::post, Router};
pub struct UserRoutes;

impl UserRoutes {
    pub fn routes() -> Router<AppState> {
        Router::new().route("/users", post(UserHandlers::create))
    }
}
