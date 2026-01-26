use anyhow::Result;
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        entities::zones::{CreateZoneEntity, UpdateZoneEntity, ZoneEntity},
        repositories::zones::ZonesRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct ZonePostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl ZonePostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ZonesRepository for ZonePostgres {
    async fn create_zone(&self, create_zone: CreateZoneEntity) -> Result<Uuid> {
        // Implementation goes here
        unimplemented!()
    }

    async fn update_zone(&self, zone_id: Uuid, zone: UpdateZoneEntity) -> Result<()> {
        // Implementation goes here
        unimplemented!()
    }

    async fn get_zone_info(&self, event_id: Uuid) -> Result<ZoneEntity> {
        // Implementation goes here
        unimplemented!()
    }

    async fn delete_zone(&self, zone_id: Uuid) -> Result<()> {
        // Implementation goes here
        unimplemented!()
    }
}
