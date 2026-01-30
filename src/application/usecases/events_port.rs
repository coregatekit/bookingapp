use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::value_objects::event_model::CreateEventModel;

#[async_trait]
#[automock]
pub trait EventsPort {
    async fn create(&self, create_event_model: CreateEventModel) -> Result<Uuid>;
}
