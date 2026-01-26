use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::postgres::schema::events;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventEntity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub performer: String,
    pub date: DateTime<Utc>,
    pub location: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = events)]
pub struct CreateEventEntity {
  pub name: String,
  pub description: Option<String>,
  pub performer: String,
  pub date: DateTime<Utc>,
  pub location: String,
  pub status: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
