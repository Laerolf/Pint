use axum::Router;
use infrastructure::features::{
    customer::repository::CustomerDatabaseRepository, location::repository::VenueDatabaseRepository,
};
use sea_orm::DatabaseConnection;

use crate::shared::context::ApiContext;

pub mod context;
pub mod environment;
pub mod error;

/// Represents an API feature.
pub trait ApiFeature {
    /// Returns the routes of this feature.
    fn routes()
    -> Router<ApiContext<DatabaseConnection, VenueDatabaseRepository, CustomerDatabaseRepository>>;
}
