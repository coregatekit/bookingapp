use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::zones::{CreateZoneEntity, ZoneEntity};

#[async_trait]
#[automock]
pub trait ZonesRepository {
    async fn create_zone(&self, create_zone: CreateZoneEntity) -> Result<Uuid>;
    async fn create_zones(
        &self,
        event_id: Uuid,
        create_zones: Vec<CreateZoneEntity>,
    ) -> Result<Vec<ZoneEntity>>;
    async fn get_zone_info(&self, event_id: Uuid) -> Result<ZoneEntity>;
}
