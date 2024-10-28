use super::{CreateInput, ListInput, Result, UserAction};
use crate::AppState;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

pub struct UserHandlers;

impl UserHandlers {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateInput>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::Create(payload).execute(&state).await?;

        Ok(response)
    }
    pub async fn list(
        State(state): State<AppState>,
        Query(params): Query<ListInput>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::List(params).execute(&state).await?;
        Ok(response)
    }
}
