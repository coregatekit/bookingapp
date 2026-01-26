use anyhow::{Ok, Result};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper,
    dsl::{exists, select},
    insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::{
        entities::events::{CreateEventEntity, EventEntity},
        repositories::events::EventsRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::events},
};

pub struct EventPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl EventPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl EventsRepository for EventPostgres {
    async fn create(&self, create_event: CreateEventEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(events::table)
            .values(create_event)
            .returning(events::id)
            .get_result(&mut conn)?;

        Ok(result)
    }

    async fn get_event_info(&self, id: Uuid) -> Result<EventEntity> {
        let mut conn = self.db_pool.get()?;

        let result = events::table
            .filter(events::id.eq(id))
            .select(EventEntity::as_select())
            .first::<EventEntity>(&mut conn)?;

        Ok(result)
    }

    async fn check_existence(&self, id: Uuid) -> Result<bool> {
        let mut conn = self.db_pool.get()?;

        let result: bool =
            select(exists(events::table.filter(events::id.eq(id)))).get_result::<bool>(&mut conn)?;

        Ok(result)
    }
}
