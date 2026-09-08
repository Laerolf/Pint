use axum::{Json, Router, extract::State, routing::get};
use domain::{
    features::customer::{dto::CustomerDto, error::CustomerErrorKind},
    shared::error::DomainError,
};
use infrastructure::features::customer::repository::CustomerDatabaseRepository;
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::shared::{ApiFeature, context::ApiContext, error::AppError};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> Router<ApiContext<DatabaseConnection, CustomerDatabaseRepository>> {
        Router::new().route("/", get(get_all_customers))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_all_customers),
    tags((name = "Customers"))
)]
pub struct CustomersApiDoc;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/customers",
    responses(
        (status = 200, description = "The customers were found.", body = Vec<CustomerDto>),
    ),
    security(("api_auth" = [])),
    tag = "Customers"
)]
async fn get_all_customers(
    State(context): State<ApiContext<DatabaseConnection, CustomerDatabaseRepository>>,
) -> Result<Json<Vec<CustomerDto>>, AppError> {
    let all_customers = context
        .customer_query_service()
        .get_all(context.db_connection())
        .await?
        .iter()
        .map(CustomerDto::from)
        .collect::<Result<Vec<CustomerDto>, DomainError<CustomerErrorKind>>>()?;

    Ok(Json(all_customers))
}

#[cfg(test)]
mod tests {

    mod get_all_customers {
        use axum::{
            body::Body,
            http::{Request, StatusCode},
        };
        use domain::shared::Source;
        use entity::customers;
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
        use std::sync::Arc;
        use tower::ServiceExt;

        use super::super::*;

        #[tokio::test]
        async fn test_get_all_customers_returns_200_with_customers() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![customers::Model {
                    id: 1,
                    first_name: Some("John".to_string()),
                    last_name: Some("Osbourne".to_string()),
                    nickname: Some("Ozzy".to_string()),
                    email_address: Some("ozzy@in.heaven".to_string()),
                    date_of_birth: None,
                    source: Some(Source::Square.to_string()),
                    source_id: Some("CUST123".to_string()),
                    created_at: chrono::Utc::now().naive_utc(),
                    last_updated_at: None,
                }]])
                .into_connection();

            let context = ApiContext::new(
                Arc::new(db_connection),
                CustomerDatabaseRepository::default(),
            );

            let router = Feature::routes().with_state(context);

            // When
            let response = router
                .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                .await
                .unwrap();

            // Then
            assert_eq!(response.status(), StatusCode::OK);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let customers: Vec<CustomerDto> = serde_json::from_slice(&body).unwrap();

            assert_eq!(customers.len(), 1);
        }

        #[tokio::test]
        async fn test_get_all_customers_returns_an_error_when_the_database_fails() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_errors([DbErr::Custom("Test".to_string())])
                .into_connection();

            let context = ApiContext::new(
                Arc::new(db_connection),
                CustomerDatabaseRepository::default(),
            );

            let router = Feature::routes().with_state(context);

            // When
            let response = router
                .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                .await
                .unwrap();

            // Then
            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        }

        #[tokio::test]
        async fn test_get_all_customers_returns_empty_list_when_no_customers_exist() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([Vec::<customers::Model>::new()])
                .into_connection();

            let context = ApiContext::new(
                Arc::new(db_connection),
                CustomerDatabaseRepository::default(),
            );

            let router = Feature::routes().with_state(context);

            // When
            let response = router
                .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                .await
                .unwrap();

            // Then
            assert_eq!(response.status(), StatusCode::OK);

            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let customers: Vec<CustomerDto> = serde_json::from_slice(&body).unwrap();

            assert!(customers.is_empty());
        }
    }
}
