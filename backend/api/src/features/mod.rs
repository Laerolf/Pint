use axum::Router;
use infrastructure::features::{
    customer::repository::CustomerDatabaseRepository, location::repository::VenueDatabaseRepository,
};
use sea_orm::DatabaseConnection;
use utoipa::{
    OpenApi,
    openapi::{
        ComponentsBuilder, InfoBuilder, OpenApi as OpenApiDoc, OpenApiBuilder,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use crate::{
    features::{
        customer::endpoints::{CustomersApiDoc, Feature as CustomersFeature},
        venue::endpoints::{Feature as VenuesFeature, VenuesApiDoc},
    },
    shared::{ApiFeature, context::ApiContext},
};

pub mod customer;
pub mod venue;

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
        .merge_from(VenuesApiDoc::openapi())
        .merge_from(CustomersApiDoc::openapi())
}

pub fn routes()
-> Router<ApiContext<DatabaseConnection, VenueDatabaseRepository, CustomerDatabaseRepository>> {
    Router::new()
        .nest("/venues", VenuesFeature::routes())
        .nest("/customers", CustomersFeature::routes())
}
