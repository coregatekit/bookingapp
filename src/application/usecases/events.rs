use std::sync::Arc;

use crate::domain::repositories::events::EventsRepository;

pub struct EventsUseCase<T>
where
    T: EventsRepository + Send + Sync,
{
    events_repository: Arc<T>,
}

impl<T> EventsUseCase<T>
where
    T: EventsRepository + Send + Sync,
{
    pub fn new(events_repository: Arc<T>) -> Self {
        Self { events_repository }
    }
}
