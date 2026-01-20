use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::postgres::schema::events;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = events)]
pub struct EventEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub performer: String,
    pub date: NaiveDateTime,
    pub location: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
