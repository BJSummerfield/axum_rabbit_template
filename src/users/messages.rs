use super::{Error, Result, UserResponse};
use lapin::{options::*, BasicProperties, Channel};

impl UserResponse {
    pub async fn publish(&self, rabbit_channel: &Channel) -> Result<()> {
        let (routing_key, message) = self.prepare_publish_data()?;

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

    fn prepare_publish_data(&self) -> Result<(&str, String)> {
        match self {
            UserResponse::Create(user) => Ok(("user.created", serde_json::to_string(user)?)),
            UserResponse::Update(user) => Ok(("user.updated", serde_json::to_string(user)?)),
            UserResponse::Delete(deleted_user) => {
                Ok(("user.deleted", serde_json::to_string(deleted_user)?))
            }
            _ => Err(Error::ValidationError(
                "Invalid context for publishing message".into(),
            )),
        }
    }
}
