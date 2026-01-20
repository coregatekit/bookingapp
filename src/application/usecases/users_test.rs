#[cfg(test)]
mod test {
    use std::sync::Arc;

    use anyhow::Ok;
    use chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::{application::usecases::users::UsersUseCase, domain::{entities::users::UserEntity, repositories::users::MockUsersRepository}};

    #[tokio::test]
    async fn test_find_user() {
        let mut mock_user_repo = MockUsersRepository::new();

        mock_user_repo.expect_find_by_email().returning(|_| {
            Box::pin(async {
                Ok(UserEntity {
                    id: Uuid::now_v7(),
                    name: "Uchinaga Aeri".to_string(),
                    email: "aerichandesu@example.com".to_string(),
                    mobile_phone: "090-1234-5678".to_string(),
                    gender: "Female".to_string(),
                    created_at: NaiveDateTime::parse_from_str(
                        "2026-01-04 11:47:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        "2026-01-04 11:47:02",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                })
            })
        });

        let use_case = UsersUseCase::new(Arc::new(mock_user_repo));

        let result = use_case.find_by_email("aerichandesu@example.com".to_string()).await;

        assert!(result.is_ok());
        assert!(result.unwrap().name == "Uchinaga Aeri".to_string());
    }
}
