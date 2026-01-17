use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::users::UserEntity;

#[async_trait]
#[automock]
pub trait UsersRepository {
    async fn find_by_username(&self, username: String) -> Result<UserEntity>;
}
