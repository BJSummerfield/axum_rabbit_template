use super::{CreateInput, Result, UserAction};
use crate::AppState;
use axum::{extract::State, response::IntoResponse, Json};

pub struct UserHandlers;

impl UserHandlers {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateInput>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::Create(payload).execute(&state).await?;

        Ok(response)
    }
}
