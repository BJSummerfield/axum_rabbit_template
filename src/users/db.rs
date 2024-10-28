use super::{CreateInput, ListInput, Result, SortOrder, User, UserList};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::ToSql, NoTls};

impl User {
    pub async fn create(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        input: CreateInput,
    ) -> Result<User> {
        let mut conn = db_pool.get().await?;
        let tx = conn.transaction().await?;
        let row = tx
            .query_one(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
                &[&input.name, &input.email],
            )
            .await?;

        let user = User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };

        tx.commit().await?;
        Ok(user)
    }

    pub async fn list(
        db_pool: &Pool<PostgresConnectionManager<NoTls>>,
        params: ListInput,
    ) -> Result<UserList> {
        let conn = db_pool.get().await?;

        let mut query = String::from("SELECT id, name, email FROM users");
        let mut query_params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        let sort_by = params.sort_by.as_deref().unwrap_or("id");
        let order = params.sort_order.unwrap_or(SortOrder::Asc);
        query.push_str(&format!(" ORDER BY {} {}", sort_by, order));

        let default_limit = 10;
        let max_limit = 50;
        let limit = params.limit.unwrap_or(default_limit).min(max_limit);
        let offset = params.offset.unwrap_or(0);
        let adjusted_limit = limit + 1;
        query.push_str(" LIMIT $1 OFFSET $2");
        query_params.push(&(adjusted_limit) as &(dyn ToSql + Sync)); // Request limit + 1
        query_params.push(&offset as &(dyn ToSql + Sync));

        let rows = conn.query(&query, &query_params).await?;

        // Check if there are more records by verifying if we have more than `limit` records.
        let has_more = rows.len() as i64 > limit;
        let users: Vec<User> = rows
            .into_iter()
            .take(limit as usize) // Take only the `limit` number of rows
            .map(|row| User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
            })
            .collect();

        let next_offset = if has_more { Some(offset + limit) } else { None };

        Ok(UserList {
            data: users,
            current_offset: offset,
            next_offset,
            limit,
            has_more,
        })
    }
}
