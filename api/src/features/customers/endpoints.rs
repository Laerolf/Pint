use axum::{Json, Router, extract::State, routing::get};
use domain::features::customer::dto::CustomerDto;
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::shared::{ApiFeature, context::ApiContext, error::AppError};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> Router<ApiContext<DatabaseConnection>> {
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
    State(context): State<ApiContext<DatabaseConnection>>,
) -> Result<Json<Vec<CustomerDto>>, AppError> {
    Ok(Json(vec![]))
}
