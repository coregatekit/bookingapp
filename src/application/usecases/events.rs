use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::{
    application::usecases::zones_port::ZonesPort,
    domain::{
        repositories::events::EventsRepository, value_objects::event_model::CreateEventModel,
    },
};

pub struct EventsUseCase<ER, ZU>
where
    ER: EventsRepository + Send + Sync,
    ZU: ZonesPort + Send + Sync,
{
    events_repository: Arc<ER>,
    zones_usecase: Arc<ZU>,
}

impl<ER, ZU> EventsUseCase<ER, ZU>
where
    ER: EventsRepository + Send + Sync,
    ZU: ZonesPort + Send + Sync,
{
    pub fn new(events_repository: Arc<ER>, zones_usecase: Arc<ZU>) -> Self {
        Self {
            events_repository,
            zones_usecase,
        }
    }

    pub async fn create(&self, create_event_model: CreateEventModel) -> Result<Uuid> {
        let create_event_entity = create_event_model.to_entity()?;

        let event_id = self.events_repository.create(create_event_entity).await?;

        // Now you can call zones_usecase methods:
        // self.zones_usecase.create_zones(event_id, zones).await?;

        Ok(event_id)
    }
}
