use crate::{
    domain::{entities::zones::ZoneEntity, value_objects::zone_model::CreateZoneModel},
};

use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

#[automock]
#[async_trait]
pub trait ZonesPort {
    async fn create_zones(&self, event_id: Uuid, create_zone_models: Vec<CreateZoneModel>) -> Result<Vec<ZoneEntity>>;
}
