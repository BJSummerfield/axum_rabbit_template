use super::Result;
use crate::AppState;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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
#[serde(rename_all = "snake_case")]
pub enum UserFields {
    Id,
    Name,
    Email,
}

impl std::fmt::Display for UserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column_name = match self {
            UserFields::Id => "id",
            UserFields::Name => "name",
            UserFields::Email => "email",
        };
        write!(f, "{}", column_name)
    }
}

#[derive(Debug, Deserialize)]
pub enum UserAction {
    Create(CreateInput),
    Get(i32),
    Update(UpdateInput),
    Delete(i32),
    List(ListInput<UserFields>),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateInput {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateInput {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
}

impl UpdateInput {
    pub fn fields_to_update(&self) -> Vec<(UserFields, Option<&str>)> {
        vec![
            (UserFields::Name, self.name.as_deref()),
            (UserFields::Email, self.email.as_deref()),
        ]
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListInput<T> {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<T>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SortOrder {
    Asc,
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

#[derive(Serialize)]
pub enum UserResponse {
    Create(User),
    Get(User),
    List(UserList),
    Update(User),
    Delete(i32),
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        match self {
            UserResponse::Create(user) => (StatusCode::CREATED, Json(user)).into_response(),
            UserResponse::Get(user) => (StatusCode::OK, Json(user)).into_response(),
            UserResponse::List(user_list) => (StatusCode::OK, Json(user_list)).into_response(),
            UserResponse::Update(user) => (StatusCode::OK, Json(user)).into_response(),
            UserResponse::Delete(_) => (StatusCode::NO_CONTENT).into_response(),
        }
    }
}

impl UserAction {
    pub async fn execute(self, state: &AppState) -> Result<UserResponse> {
        match self {
            UserAction::Create(input) => {
                let response = User::create(&state.db_pool, input).await?;
                Ok(response)
            }
            UserAction::Get(input) => {
                let response = User::get(&state.db_pool, input).await?;
                Ok(response)
            }
            UserAction::List(input) => {
                let response = User::list(&state.db_pool, input).await?;
                Ok(response)
            }
            UserAction::Update(input) => {
                let response = User::update(&state.db_pool, input).await?;
                Ok(response)
            }
            UserAction::Delete(input) => {
                let response = User::delete(&state.db_pool, input).await?;
                Ok(response)
            }
        }
    }
}
