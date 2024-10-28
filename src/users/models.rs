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

#[derive(Debug, Serialize)]
pub struct UserList {
    pub data: Vec<User>,
    pub current_offset: i64,
    pub next_offset: Option<i64>,
    pub limit: i64,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub enum UserAction {
    Create(CreateInput),
    // Get(GetInput),
    // Update(UpdateInput),
    // Delete(DeleteInput),
    List(ListInput),
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
#[derive(Deserialize, Debug)]
pub struct ListInput {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Deserialize)]
pub enum SortOrder {
    #[serde(alias = "asc", alias = "Asc", alias = "ASC")]
    Asc,
    #[serde(alias = "desc", alias = "Desc", alias = "DESC")]
    Desc,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };
        write!(f, "{}", s)
    }
}

impl UserAction {
    pub async fn execute(self, state: &AppState) -> Result<impl IntoResponse> {
        match self {
            UserAction::Create(input) => {
                let user = User::create(&state.db_pool, input).await?;
                user.publish(&state.rabbit_channel).await?;
                Ok((StatusCode::CREATED, Json(user)).into_response())
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
            UserAction::List(input) => {
                let user_list: UserList = User::list(&state.db_pool, input).await?;
                Ok((StatusCode::OK, Json(user_list)).into_response())
            }
        }
    }
}
