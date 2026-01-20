use std::sync::Arc;

use anyhow::Result;

use crate::domain::{entities::users::UserEntity, repositories::users::UsersRepository};

pub struct UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    users_repository: Arc<T>,
}

impl<T> UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self {
        Self { users_repository }
    }

    pub async fn find_by_email(&self, email: String) -> Result<UserEntity> {
        let user = self.users_repository.find_by_email(email).await?;
        Ok(user)
    }
}
