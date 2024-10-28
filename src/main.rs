mod state;
mod users;

use axum::Router;
use state::AppState;
use users::UserRoutes;

#[tokio::main]
async fn main() {
    let state = AppState::new().await;

    let app = Router::new().merge(UserRoutes::routes()).with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
