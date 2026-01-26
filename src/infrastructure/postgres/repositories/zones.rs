use anyhow::Result;
use diesel::{RunQueryDsl, insert_into};
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        entities::zones::{CreateZoneEntity, ZoneEntity},
        repositories::zones::ZonesRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::zones},
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
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(zones::table)
            .values(create_zone)
            .returning(zones::id)
            .get_result(&mut conn)?;

        Ok(result)
    }

    async fn get_zone_info(&self, event_id: Uuid) -> Result<ZoneEntity> {
        // Implementation goes here
        unimplemented!()
    }
}
