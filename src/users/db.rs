use super::{
    CreateInput, DeletedUser, Error, ListInput, Result, SortOrder, UpdateInput, User, UserFields,
    UserList, UserResponse,
};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls};

impl User {
    pub async fn create(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        input: CreateInput,
    ) -> Result<UserResponse> {
        let mut conn = db_pool.get().await?;
        let tx = conn.transaction().await?;
        let row = tx
            .query_one(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
                &[&input.name, &input.email],
            )
            .await?;

        let user = User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
        };

        tx.commit().await?;
        Ok(UserResponse::Create(user))
    }

    pub async fn get(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        id: i32,
    ) -> Result<UserResponse> {
        let conn = db_pool.get().await?;

        let row = conn
            .query_opt("SELECT id, name, email FROM users WHERE id = $1", &[&id])
            .await?;

        if let Some(row) = row {
            Ok(UserResponse::Get(User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            }))
        } else {
            Err(Error::NotFound(format!("User with id {} not found", id)))
        }
    }

    pub async fn list(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        params: ListInput<UserFields>,
    ) -> Result<UserResponse> {
        let conn = db_pool.get().await?;

        let mut query = String::from("SELECT id, name, email FROM users");
        let mut query_params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let sort_by = params.sort_by.unwrap_or(UserFields::Id).to_string();
        let order = params.sort_order.unwrap_or(SortOrder::Asc);
        query.push_str(&format!(" ORDER BY {} {}", sort_by, order));

        let default_limit = 10;
        let max_limit = 50;
        let limit = params.limit.unwrap_or(default_limit);

        if limit > max_limit {
            return Err(Error::ValidationError(format!(
                "Requested limit ({}) exceeds the maximum allowed ({})",
                limit, max_limit
            )));
        }

        let offset = params.offset.unwrap_or(0);
        let adjusted_limit = limit + 1;
        query.push_str(" LIMIT $1 OFFSET $2");
        query_params.push(&(adjusted_limit) as &(dyn ToSql + Sync));
        query_params.push(&offset as &(dyn ToSql + Sync));

        let rows = conn.query(&query, &query_params).await?;

        let has_more = rows.len() as i64 > limit;
        let users: Vec<User> = rows
            .into_iter()
            .take(limit as usize)
            .map(|row| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            })
            .collect();

        let next_offset = if has_more { Some(offset + limit) } else { None };

        Ok(UserResponse::List(UserList {
            data: users,
            current_offset: offset,
            next_offset,
            limit,
            has_more,
        }))
    }

    pub async fn update(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        input: UpdateInput,
    ) -> Result<UserResponse> {
        let mut conn = db_pool.get().await?;
        let transaction = conn.transaction().await?;
        let mut updates = Vec::new();
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut param_index = 1;

        let mut field_values: Vec<String> = Vec::new();

        for (field, value) in input.fields_to_update() {
            if let Some(val) = value {
                match field {
                    UserFields::Id => continue,
                    _ => {
                        updates.push(format!("{} = ${}", field, param_index));
                        field_values.push(val.to_string());
                        param_index += 1;
                    }
                }
            }
        }

        for field_value in &field_values {
            params.push(field_value as &(dyn ToSql + Sync));
        }

        if updates.is_empty() {
            return Err(Error::ValidationError("No fields to update.".to_string()));
        }

        let query = format!(
            "UPDATE users SET {} WHERE id = ${}",
            updates.join(", "),
            param_index
        );
        params.push(&input.id);

        let rows_affected = transaction.execute(&query, &params).await?;

        if rows_affected == 0 {
            transaction.rollback().await?;
            return Err(Error::NotFound(format!(
                "User with id {} not found",
                input.id
            )));
        }

        let row = transaction
            .query_one(
                "SELECT id, name, email FROM users WHERE id = $1",
                &[&input.id],
            )
            .await?;

        transaction.commit().await?;
        Ok(UserResponse::Update(User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
        }))
    }

    pub async fn delete(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        id: i32,
    ) -> Result<UserResponse> {
        let conn = db_pool.get().await?;

        let rows_affected = conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .await?;

        if rows_affected == 0 {
            return Err(Error::NotFound(format!("User with id {} not found", id)));
        }

        Ok(UserResponse::Delete(DeletedUser { id }))
    }
}
