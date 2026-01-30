use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::{
    domain::{
        repositories::events::EventsRepository, value_objects::event_model::CreateEventModel,
    },
};

pub struct EventsUseCase<ER>
where
    ER: EventsRepository + Send + Sync,
{
    events_repository: Arc<ER>,
}

impl<ER> EventsUseCase<ER>
where
    ER: EventsRepository + Send + Sync,
{
    pub fn new(events_repository: Arc<ER>) -> Self {
        Self { events_repository }
    }

    pub async fn create(&self, create_event_model: CreateEventModel) -> Result<Uuid> {
        let create_event_entity = create_event_model.to_entity()?;

        let event_id = self.events_repository.create(create_event_entity).await?;

        // Now you can call zones_usecase methods:
        // self.zones_usecase.create_zones(event_id, zones).await?;

        Ok(event_id)
    }
}
