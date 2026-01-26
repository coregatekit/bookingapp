use crate::domain::repositories::{events::EventsRepository, zones::ZonesRepository};

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
}
