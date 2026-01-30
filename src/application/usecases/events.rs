use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::usecases::{events_port::EventsPort, zones_port::ZonesPort},
    domain::{
        repositories::events::EventsRepository, value_objects::event_model::CreateEventModel,
    },
};

pub struct EventsUseCase<ER, ZP>
where
    ER: EventsRepository + Send + Sync,
    ZP: ZonesPort + Send + Sync,
{
    events_repository: Arc<ER>,
    zones_port: Arc<ZP>,
}

impl<ER, ZP> EventsUseCase<ER, ZP>
where
    ER: EventsRepository + Send + Sync,
    ZP: ZonesPort + Send + Sync,
{
    pub fn new(events_repository: Arc<ER>, zones_port: Arc<ZP>) -> Self {
        Self {
            events_repository,
            zones_port,
        }
    }
}

#[async_trait]
impl<ER, ZP> EventsPort for EventsUseCase<ER, ZP>
where
    ER: EventsRepository + Send + Sync,
    ZP: ZonesPort + Send + Sync,
{
    async fn create(&self, create_event_model: CreateEventModel) -> Result<Uuid> {
        let create_event_entity = create_event_model.to_entity()?;

        let event_id = self.events_repository.create(create_event_entity).await?;

        if let Some(zone_models) = create_event_model.create_zone {
            self.zones_port.create_zones(event_id, zone_models).await?;
        }

        Ok(event_id)
    }
}
