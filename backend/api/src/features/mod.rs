use axum::Router;
use infrastructure::features::customer::repository::CustomerDatabaseRepository;
use sea_orm::DatabaseConnection;
use utoipa::{
    OpenApi,
    openapi::{
        ComponentsBuilder, InfoBuilder, OpenApi as OpenApiDoc, OpenApiBuilder,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use crate::{
    features::customers::endpoints::{CustomersApiDoc, Feature as CustomersFeature},
    shared::{ApiFeature, context::ApiContext},
};

pub mod customers;

pub fn openapi() -> OpenApiDoc {
    OpenApiBuilder::new()
        .info(InfoBuilder::new().title("Pint API").build())
        .components(Some(
            ComponentsBuilder::new()
                .security_scheme(
                    "api_auth",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        ))
        .build()
        .merge_from(CustomersApiDoc::openapi())
}

pub fn routes() -> Router<ApiContext<DatabaseConnection, CustomerDatabaseRepository>> {
    Router::new().nest("/customers", CustomersFeature::routes())
}
