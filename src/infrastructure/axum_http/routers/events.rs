use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::usecases::{
        events::EventsUseCase, events_port::EventsPort, zones::ZonesUseCase, zones_port::ZonesPort,
    },
    domain::value_objects::event_model::CreateEventModel,
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{events::EventPostgres, zones::ZonePostgres},
    },
};

#[derive(Clone)]
pub struct EventsAppState<E, Z>
where
    E: EventsPort + Send + Sync,
    Z: ZonesPort + Send + Sync,
{
    pub events_usecase: Arc<E>,
    pub zones_usecase: Arc<Z>,
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let events_repository = Arc::new(EventPostgres::new(db_pool.clone()));
    let zones_repository = Arc::new(ZonePostgres::new(db_pool.clone()));

    let zones_usecase = Arc::new(ZonesUseCase::new(
        zones_repository.clone(),
        events_repository.clone(),
    ));
    let events_usecase = Arc::new(EventsUseCase::new(
        events_repository.clone(),
        zones_usecase.clone(),
    ));

    let state = EventsAppState {
        events_usecase,
        zones_usecase,
    };

    Router::new()
        .route("/", post(create_event))
        .with_state(Arc::new(state))
}

pub async fn create_event<E, Z>(
    State(state): State<Arc<EventsAppState<E, Z>>>,
    Json(payload): Json<CreateEventModel>,
) -> impl IntoResponse
where
    E: EventsPort + Send + Sync,
    Z: ZonesPort + Send + Sync,
{
    println!("Received payload: {}", payload.name);

    // let result = state.events_usecase.create(payload).await?;
    match state.events_usecase.create(payload).await {
        Ok(event_id) => (StatusCode::CREATED, Json(event_id)).into_response(),
        Err(e) => {
            eprintln!("Error creating event: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to create event".to_string()),
            )
                .into_response()
        }
    }
}
