use super::Result;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use lapin::{options::*, BasicProperties, Channel};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

impl CreateUser {
    pub async fn create_pg(self, db_pool: &Pool<PostgresConnectionManager<NoTls>>) -> Result<User> {
        let mut conn = db_pool.get().await?;
        let tx = conn.transaction().await?;
        let row = tx
            .query_one(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
                &[&self.name, &self.email],
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

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn publish_rabbit(&self, rabbit_channel: &Channel) -> Result<()> {
        let message = serde_json::to_string(self)?;
        rabbit_channel
            .basic_publish(
                "user_events",
                "user.created",
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }
}
