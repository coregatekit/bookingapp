#[cfg(test)]
mod test {
    use std::sync::Arc;

    use anyhow::Ok;
    use uuid::Uuid;

    use crate::{
        application::usecases::events::EventsUseCase,
        domain::{repositories::events::MockEventsRepository, value_objects::event_model::CreateEventModel},
    };

    #[tokio::test]
    async fn test_create_event() {
        let mut mock_event_repo = MockEventsRepository::new();
        let mock_event_id = Uuid::now_v7();
        let mock_create_event_model = CreateEventModel {
            name: "Test Event".to_string(),
            description: Some("This is a test event.".to_string()),
            performer: "Test Performer".to_string(),
            date: "2026-02-01T10:00:00Z".to_string(),
            location: "Test Location".to_string(),
            create_zone: None,
        };

        mock_event_repo
            .expect_create()
            .returning(move |_| Box::pin(async move { Ok(mock_event_id) }));

        let use_case = EventsUseCase::new(Arc::new(mock_event_repo));

        let result = use_case.create(mock_create_event_model).await;

        assert!(result.is_ok());
        assert!(result.unwrap() == mock_event_id);
    }
}
