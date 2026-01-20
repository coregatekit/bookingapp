use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::{
    application::usecases::users::UsersUseCase,
    domain::repositories::users::UsersRepository,
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::users::UserPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let users_repository = UserPostgres::new(db_pool);
    let users_usecase = UsersUseCase::new(Arc::new(users_repository));

    Router::new().route("/find/:email", get(find_by_email)).with_state(Arc::new(users_usecase))
}

pub async fn find_by_email<T>(
    State(users_usecase): State<Arc<UsersUseCase<T>>>,
    Path(email): Path<String>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match users_usecase.find_by_email(email).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, Json(e.to_string())).into_response(),
    }
}
