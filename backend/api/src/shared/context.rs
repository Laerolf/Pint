use std::sync::Arc;

use domain::features::{
    customer::{repository::CustomerRepository, service::query::CustomerQueryService},
    venue::{repository::VenueRepository, service::query::VenueQueryService},
};
use sea_orm::ConnectionTrait;

/// Represents the context of the API.
#[derive(Clone)]
pub struct ApiContext<C: ConnectionTrait, VR: VenueRepository, CR: CustomerRepository> {
    db_connection: Arc<C>,
    venue_query_service: VenueQueryService<VR>,
    customer_query_service: CustomerQueryService<CR>,
}

impl<C: ConnectionTrait, VR: VenueRepository, CR: CustomerRepository> ApiContext<C, VR, CR> {
    /// Creates a new [`Context`].
    pub fn new(db_connection: Arc<C>, venue_repository: VR, customer_repository: CR) -> Self {
        let venue_query_service = VenueQueryService::new(venue_repository);
        let customer_query_service = CustomerQueryService::new(customer_repository);

        Self {
            db_connection,
            venue_query_service,
            customer_query_service,
        }
    }

    pub fn db_connection(&self) -> &C {
        &self.db_connection
    }

    pub fn venue_query_service(&self) -> &VenueQueryService<VR> {
        &self.venue_query_service
    }

    pub fn customer_query_service(&self) -> &CustomerQueryService<CR> {
        &self.customer_query_service
    }
}
