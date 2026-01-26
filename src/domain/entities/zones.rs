use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::postgres::schema::zones;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = zones)]
pub struct ZoneEntity {
    pub id: Uuid,
    pub event_id: Uuid,
    pub label: String,
    pub price: BigDecimal,
    pub total_seats: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = zones)]
pub struct CreateZoneEntity {
    pub event_id: Uuid,
    pub label: String,
    pub price: BigDecimal,
    pub total_seats: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = zones)]
pub struct UpdateZoneEntity {
    pub label: Option<String>,
    pub price: Option<BigDecimal>,
    pub total_seats: Option<i32>,
    pub updated_at: NaiveDateTime,
}
