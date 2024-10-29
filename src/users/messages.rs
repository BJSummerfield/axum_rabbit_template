use super::{Error, Result, UserResponse};
use lapin::{options::*, BasicProperties, Channel};

impl UserResponse {
    pub async fn publish(&self, rabbit_channel: &Channel) -> Result<()> {
        let message = self.build_message()?;
        let routing_key = self.generate_routing_key()?;

        rabbit_channel
            .basic_publish(
                "user_events",
                routing_key,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;

        println!("Published message: {}", message);
        Ok(())
    }

    fn build_message(&self) -> Result<String> {
        match self {
            UserResponse::Create(user) | UserResponse::Update(user) | UserResponse::Get(user) => {
                Ok(serde_json::to_string(user)?)
            }
            _ => Err(Error::ValidationError(
                "Invalid context for publishing message".into(),
            )),
        }
    }

    fn generate_routing_key(&self) -> Result<&str> {
        match self {
            UserResponse::Create(_) => Ok("user.created"),
            UserResponse::Update(_) => Ok("user.updated"),
            UserResponse::Delete(_) => Ok("user.deleted"),
            _ => Err(Error::ValidationError(
                "Invalid context for generating routing key".into(),
            )),
        }
    }
}
