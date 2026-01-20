use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{entities::users::UserEntity, repositories::users::UsersRepository},
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::users},
};

pub struct UserPostgres {
    db_pool: Arc<PgPoolSquad>,
}
impl UserPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UsersRepository for UserPostgres {
    async fn find_by_email(&self, email: String) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
            .filter(users::email.eq(email))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }
}
