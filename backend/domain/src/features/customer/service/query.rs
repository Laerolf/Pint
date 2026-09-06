use sea_orm::ConnectionTrait;

use crate::{
    features::customer::{
        domain::Customer, error::CustomerErrorKind, mapper::CustomerMapper,
        repository::CustomerRepository,
    },
    shared::error::DomainError,
};

/// Represents a query service dealing with [`Customers`][Customer].
#[derive(Clone)]
pub struct CustomerQueryService<R: CustomerRepository> {
    repository: R,
}

impl<R: CustomerRepository> CustomerQueryService<R> {
    /// Creates a new [`CustomerQueryService`].
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Retrieves all existing [`Customers`][Vec<Customer>].
    pub async fn get_all<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<Customer>, DomainError<CustomerErrorKind>> {
        self.repository
            .get_all(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::GetAll).with_cause(error))?
            .iter()
            .map(CustomerMapper::to_domain_model)
            .collect()
    }
}
