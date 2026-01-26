#[cfg(test)]
mod test {
    use std::sync::Arc;

    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        application::usecases::zones::ZonesUseCase,
        domain::{
            entities::zones::ZoneEntity,
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

    #[tokio::test]
    async fn test_should_throw_an_error_when_event_does_not_exist() {
        let mut mock_event_repo = MockEventsRepository::new();
        let mock_zone_repo = MockZonesRepository::new();

        let event_id = Uuid::now_v7();
        let expected_event_id = event_id;

        mock_event_repo
            .expect_check_existence()
            .withf(move |id| *id == expected_event_id)
            .returning(|_| Box::pin(async { Ok(false) }));

        let use_case = ZonesUseCase::new(Arc::new(mock_zone_repo), Arc::new(mock_event_repo));

        let mut create_zone_models = Vec::new();
        create_zone_models.push(CreateZoneModel {
            label: "VIP".to_string(),
            price: "150.00".parse().unwrap(),
            total_seats: 50,
        });

        let result = use_case.create_zone(event_id, create_zone_models).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_zone_successfully() {
        let mut mock_event_repo = MockEventsRepository::new();
        let mut mock_zone_repo = MockZonesRepository::new();

        let event_id = Uuid::now_v7();
        let now = Utc::now();
        mock_event_repo
            .expect_check_existence()
            .returning(|_| Box::pin(async { Ok(true) }));
        mock_zone_repo.expect_create_zones().returning(move |_, _| {
            let now_clone = now;
            Box::pin(async move {
                Ok(vec![
                    ZoneEntity {
                        id: Uuid::now_v7(),
                        event_id: Uuid::now_v7(),
                        label: "VIP".to_string(),
                        price: "150.00".parse().unwrap(),
                        total_seats: 50,
                        created_at: now_clone,
                        updated_at: now_clone,
                    },
                    ZoneEntity {
                        id: Uuid::now_v7(),
                        event_id: Uuid::now_v7(),
                        label: "VVIP".to_string(),
                        price: "200.00".parse().unwrap(),
                        total_seats: 50,
                        created_at: now_clone,
                        updated_at: now_clone,
                    },
                ])
            })
        });

        let use_case = ZonesUseCase::new(Arc::new(mock_zone_repo), Arc::new(mock_event_repo));

        let mut create_zone_models = Vec::new();
        create_zone_models.push(CreateZoneModel {
            label: "VIP".to_string(),
            price: "150.00".parse().unwrap(),
            total_seats: 50,
        });
        create_zone_models.push(CreateZoneModel {
            label: "VVIP".to_string(),
            price: "200.00".parse().unwrap(),
            total_seats: 50,
        });

        let result = use_case.create_zone(event_id, create_zone_models).await;

        assert!(result.is_ok());
        let zones = result.unwrap();
        assert_eq!(zones.len(), 2);
    }
}
