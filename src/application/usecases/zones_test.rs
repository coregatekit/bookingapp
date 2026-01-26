#[cfg(test)]
mod test {
    use std::sync::Arc;

    use uuid::Uuid;

    use crate::{
        application::usecases::zones::ZonesUseCase,
        domain::{
            repositories::{events::MockEventsRepository, zones::MockZonesRepository},
            value_objects::zone_model::CreateZoneModel,
        },
    };

    #[tokio::test]
    async fn test_check_event_existence_before_create_zone() {
        let mut mock_event_repo = MockEventsRepository::new();
        let mock_zone_repo = MockZonesRepository::new();

        let event_id = Uuid::now_v7();
        let expected_event_id = event_id;

        mock_event_repo
            .expect_check_existence()
            .withf(move |id| *id == expected_event_id)
            .returning(|_| Box::pin(async { Ok(true) }));

        let use_case = ZonesUseCase::new(Arc::new(mock_zone_repo), Arc::new(mock_event_repo));

        let mut create_zone_models = Vec::new();
        create_zone_models.push(CreateZoneModel {
            label: "VIP".to_string(),
            price: "150.00".parse().unwrap(),
            total_seats: 50,
        });

        let result = use_case.create_zone(event_id, create_zone_models).await;

        assert!(result.is_ok());
    }
}
