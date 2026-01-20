use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::events::CreateEventEntity;

#[async_trait]
#[automock]
pub trait EventsRepository {
    async fn create(&self, create_event: CreateEventEntity) -> Result<Uuid>;
}