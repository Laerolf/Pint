use axum::{Json, Router, extract::State, routing::get};
use domain::features::venue::dto::VenueDto;
use infrastructure::features::{
    customer::repository::CustomerDatabaseRepository, location::repository::VenueDatabaseRepository,
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::shared::{ApiFeature, context::ApiContext, error::AppError};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes()
    -> Router<ApiContext<DatabaseConnection, VenueDatabaseRepository, CustomerDatabaseRepository>>
    {
        Router::new().route("/me", get(get_session_venue))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_session_venue),
    tags((name = "Venues"))
)]
pub struct VenuesApiDoc;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/venues/me",
    responses(
        (status = 200, description = "The venue for the current session is found.", body = VenueDto),
    ),
    security(("api_auth" = [])),
    tag = "Venues"
)]
async fn get_session_venue(
    State(context): State<
        ApiContext<DatabaseConnection, VenueDatabaseRepository, CustomerDatabaseRepository>,
    >,
) -> Result<Json<VenueDto>, AppError> {
    todo!()
}

#[cfg(test)]
mod tests {

    mod get_session_venue {
        use axum::{
            body::Body,
            http::{Request, StatusCode},
        };
        use domain::shared::Source;
        use entity::venues;
        use infrastructure::features::location::repository::VenueDatabaseRepository;
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
        use std::sync::Arc;
        use tower::ServiceExt;

        use super::super::*;

        #[tokio::test]
        #[ignore]
        async fn test_find_get_session_venue_returns_200_with_a_venue() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![venues::Model {
                    id: 666666,
                    name: Some(String::from("Bar Armageddon")),
                    country: Some(String::from("Hell")),
                    language_code: Some(String::from("la-HE")),
                    timezone: Some(String::from("Europe/Oslo")),
                    currency: Some(String::from("SLS")),
                    source: Some(Source::Square.to_string()),
                    source_id: Some(String::from("main")),
                    created_at: chrono::Utc::now().naive_utc(),
                    last_updated_at: None,
                }]])
                .into_connection();

            let context = ApiContext::new(
                Arc::new(db_connection),
                VenueDatabaseRepository,
                CustomerDatabaseRepository,
            );

            let router = Feature::routes().with_state(context);

            // When
            let response = router
                .oneshot(Request::builder().uri("/1").body(Body::empty()).unwrap())
                .await
                .unwrap();

            // Then
            assert_eq!(response.status(), StatusCode::OK);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let venue: Option<VenueDto> = serde_json::from_slice(&body).unwrap();

            assert!(venue.is_some());
        }

        #[tokio::test]
        #[ignore]
        async fn test_find_get_session_venue_returns_an_error_when_the_database_fails() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_errors([DbErr::Custom("Test".to_string())])
                .into_connection();

            let context = ApiContext::new(
                Arc::new(db_connection),
                VenueDatabaseRepository,
                CustomerDatabaseRepository,
            );

            let router = Feature::routes().with_state(context);

            // When
            let response = router
                .oneshot(Request::builder().uri("/1").body(Body::empty()).unwrap())
                .await
                .unwrap();

            // Then
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
