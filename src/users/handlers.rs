use super::{CreateInput, ListInput, Result, UpdateInput, UserAction, UserFields};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
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
        response.publish(&state.rabbit_channel).await?;
        Ok(response)
    }
    pub async fn get(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::Get(id).execute(&state).await?;
        Ok(response)
    }

    pub async fn list(
        State(state): State<AppState>,
        Query(params): Query<ListInput<UserFields>>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::List(params).execute(&state).await?;
        Ok(response)
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(mut payload): Json<UpdateInput>,
    ) -> Result<impl IntoResponse> {
        payload.id = id;
        let response = UserAction::Update(payload).execute(&state).await?;
        Ok(response)
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse> {
        let response = UserAction::Delete(id).execute(&state).await?;
        Ok(response)
    }
}
