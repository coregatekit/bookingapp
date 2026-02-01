use anyhow::Result;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, insert_into};
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

    async fn create_zones(
        &self,
        event_id: Uuid,
        create_zones: Vec<CreateZoneEntity>,
    ) -> Result<Vec<ZoneEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let zones_with_event_id: Vec<CreateZoneEntity> = create_zones
            .into_iter()
            .map(|mut zone| {
                zone.event_id = event_id;
                zone
            })
            .collect();

        let results = insert_into(zones::table)
            .values(&zones_with_event_id)
            .returning(ZoneEntity::as_select())
            .get_results(&mut conn)?;

        Ok(results)
    }

    async fn get_zone_info(&self, event_id: Uuid) -> Result<ZoneEntity> {
        // Implementation goes here
        unimplemented!()
    }

    async fn get_zones_by_event_id(&self, event_id: Uuid) -> Result<Vec<ZoneEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let results = zones::table
            .filter(zones::event_id.eq(event_id))
            .select(ZoneEntity::as_select())
            .load::<ZoneEntity>(&mut conn)?;

        Ok(results)
    }
}
