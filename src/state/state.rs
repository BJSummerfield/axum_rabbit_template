use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use lapin::{
    options::*, types::FieldTable, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use tokio_postgres::{Config, NoTls};
#[derive(Clone)]
pub struct AppState {
    pub rabbit_channel: Channel,
    pub db_pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl AppState {
    pub async fn new() -> Self {
        let rabbit_channel = Self::setup_rabbitmq().await;
        let db_pool = Self::setup_postgres().await;
        Self {
            rabbit_channel,
            db_pool,
        }
    }

    async fn setup_postgres() -> Pool<PostgresConnectionManager<NoTls>> {
        let mut config = Config::new();
        config
            .user("user")
            .password("password")
            .dbname("users_db")
            .host("localhost");

        let manager = PostgresConnectionManager::new(config, NoTls);
        Pool::builder()
            .max_size(15)
            .build(manager)
            .await
            .expect("Failed to create PostgreSQL connection pool")
    }

    async fn setup_rabbitmq() -> Channel {
        let connection = Connection::connect(
            "amqp://admin:admin@localhost:5672/%2f",
            ConnectionProperties::default(),
        )
        .await
        .expect("Failed to connect to RabbitMQ");

        let channel = connection
            .create_channel()
            .await
            .expect("Failed to create channel");

        channel
            .exchange_declare(
                "user_events",
                ExchangeKind::Topic,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare exchange");

        channel
    }
}
