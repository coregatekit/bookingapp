use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::postgres::schema::users;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub mobile_phone: String,
    pub gender: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
