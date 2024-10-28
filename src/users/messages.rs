use super::{Result, User};
use lapin::{options::*, BasicProperties, Channel};

impl User {
    pub async fn publish(&self, rabbit_channel: &Channel) -> Result<()> {
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

        println!("Published message: {}", message);
        Ok(())
    }
}
