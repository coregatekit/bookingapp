use chrono::NaiveDateTime;
use diesel::sql_types::Uuid;

pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub mobile_phone: String,
    pub gender: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
