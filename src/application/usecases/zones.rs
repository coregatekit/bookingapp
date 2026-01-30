use crate::{
    application::usecases::zones_port::ZonesPort,
    domain::{
        entities::zones::ZoneEntity,
        repositories::{events::EventsRepository, zones::ZonesRepository},
        value_objects::zone_model::CreateZoneModel,
    },
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use uuid::Uuid;

pub struct ZonesUseCase<ZT, ET>
where
    ZT: ZonesRepository + Send + Sync,
    ET: EventsRepository + Send + Sync,
{
    zones_repository: std::sync::Arc<ZT>,
    events_repository: std::sync::Arc<ET>,
}

impl<ZT: ZonesRepository + Send + Sync, ET: EventsRepository + Send + Sync> ZonesUseCase<ZT, ET> {
    pub fn new(
        zones_repository: std::sync::Arc<ZT>,
        events_repository: std::sync::Arc<ET>,
    ) -> Self {
        Self {
            zones_repository,
            events_repository,
        }
    }
}

#[async_trait]
impl<ZT, ET> ZonesPort for ZonesUseCase<ZT, ET>
where
    ZT: ZonesRepository + Send + Sync,
    ET: EventsRepository + Send + Sync,
{
    async fn create_zones(
        &self,
        event_id: Uuid,
        create_zone_models: Vec<CreateZoneModel>,
    ) -> Result<Vec<ZoneEntity>> {
        let event_exists = self.events_repository.check_existence(event_id).await?;

        if !event_exists {
            return Err(anyhow::anyhow!("Event does not exist"));
        }

        let create_zone_entities: Vec<_> = create_zone_models
            .iter()
            .map(|model| model.to_entity(event_id).unwrap())
            .collect();
        let zones = self
            .zones_repository
            .create_zones(event_id, create_zone_entities)
            .await?;

        Ok(zones)
    }
}
