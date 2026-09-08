use std::sync::Arc;

use domain::features::customer::{
    repository::CustomerRepository, service::query::CustomerQueryService,
};
use sea_orm::ConnectionTrait;

/// Represents the context of the API.
#[derive(Clone)]
pub struct ApiContext<C: ConnectionTrait, CR: CustomerRepository> {
    db_connection: Arc<C>,
    customer_query_service: CustomerQueryService<CR>,
}

impl<C: ConnectionTrait, CR: CustomerRepository> ApiContext<C, CR> {
    /// Creates a new [`Context`].
    pub fn new(db_connection: Arc<C>, customer_repository: CR) -> Self {
        let customer_query_service = CustomerQueryService::new(customer_repository);

        Self {
            db_connection,
            customer_query_service,
        }
    }

    pub fn db_connection(&self) -> &C {
        &self.db_connection
    }

    pub fn customer_query_service(&self) -> &CustomerQueryService<CR> {
        &self.customer_query_service
    }
}
