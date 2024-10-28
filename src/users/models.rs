use super::Result;
use crate::AppState;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub enum UserAction {
    Create(CreateInput),
    // Get(GetInput),
    // Update(UpdateInput),
    // Delete(DeleteInput),
    // List, // No associated data needed
}

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    pub name: String,
    pub email: String,
}

// #[derive(Debug, Deserialize)]
// pub struct GetInput {
//     pub id: i32,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct UpdateInput {
//     pub id: i32,
//     pub name: Option<String>,
//     pub email: Option<String>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct DeleteInput {
//     pub id: i32,
// }

impl UserAction {
    pub async fn execute(self, state: &AppState) -> Result<impl IntoResponse> {
        match self {
            UserAction::Create(input) => {
                let user = User::create(&state.db_pool, input).await?;
                user.publish(&state.rabbit_channel).await?;
                Ok((StatusCode::CREATED, Json(user)))
            } // UserAction::Get { id } => {
              //     let user = User::get_from_db(db, id).await?;
              //     Ok((StatusCode::OK, Json(user)))
              // }
              // UserAction::Update { id, name, email } => {
              //     let user = User::update_in_db(db, id, name, email).await?;
              //     Ok((StatusCode::OK, Json(user)))
              // }
              // UserAction::Delete { id } => {
              //     User::delete_from_db(db, id).await?;
              //     Ok((StatusCode::NO_CONTENT, Json("User deleted successfully")))
              // }
              // UserAction::List => {
              //     let users = User::list_from_db(db).await?;
              //     Ok((StatusCode::OK, Json(users)))
              // }
        }
    }
}
