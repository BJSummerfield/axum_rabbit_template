use super::{CreateUser, Result};
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub struct UserHandlers;

impl UserHandlers {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateUser>,
    ) -> Result<impl IntoResponse> {
        let user = CreateUser {
            name: payload.name,
            email: payload.email,
        }
        .create_pg(&state.db_pool)
        .await?;

        user.publish_rabbit(&state.rabbit_channel).await?;

        Ok((StatusCode::CREATED, Json(user)))
    }
}
