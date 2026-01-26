use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::zones::{CreateZoneEntity, UpdateZoneEntity, ZoneEntity};

#[async_trait]
#[automock]
pub trait ZonesRepository {
    async fn create_zone(&self, create_zone: CreateZoneEntity) -> Result<Uuid>;
    async fn update_zone(&self, zone_id: Uuid, zone: UpdateZoneEntity) -> Result<()>;
    async fn get_zone_info(&self, event_id: Uuid) -> Result<ZoneEntity>;
    async fn delete_zone(&self, zone_id: Uuid) -> Result<()>;
}
