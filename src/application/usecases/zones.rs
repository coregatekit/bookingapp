use crate::domain::{
    entities::zones::ZoneEntity,
    repositories::{events::EventsRepository, zones::ZonesRepository},
    value_objects::zone_model::CreateZoneModel,
};
use anyhow::{Ok, Result};
use uuid::Uuid;

pub struct ZonesUseCase<T1, T2>
where
    T1: ZonesRepository + Send + Sync,
    T2: EventsRepository + Send + Sync,
{
    zones_repository: std::sync::Arc<T1>,
    events_repository: std::sync::Arc<T2>,
}

impl<T1, T2> ZonesUseCase<T1, T2>
where
    T1: ZonesRepository + Send + Sync,
    T2: EventsRepository + Send + Sync,
{
    pub fn new(
        zones_repository: std::sync::Arc<T1>,
        events_repository: std::sync::Arc<T2>,
    ) -> Self {
        Self {
            zones_repository,
            events_repository,
        }
    }

    pub async fn create_zone(
        &self,
        event_id: Uuid,
        create_zone_models: Vec<CreateZoneModel>,
    ) -> Result<Vec<ZoneEntity>> {
        let event_exists = self.events_repository.check_existence(event_id).await?;

        if !event_exists {
            return Err(anyhow::anyhow!("Event does not exist"));
        }

        let zones = Vec::new();
        Ok(zones)
    }
}
