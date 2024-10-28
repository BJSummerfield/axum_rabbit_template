use super::{CreateInput, Result, User};

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

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
}
