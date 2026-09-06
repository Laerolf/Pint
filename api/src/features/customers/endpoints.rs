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
