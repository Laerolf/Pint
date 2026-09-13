use async_trait::async_trait;
use entity::customers;
use sea_orm::ConnectionTrait;

use crate::{features::customer::error::CustomerErrorKind, shared::error::DomainError};

/// Represents a repository dealing with [`Customers`][customers::Model].
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// s a [`Customer`][customers::Model] with the provided ID.
    async fn find_by_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        id: &i32,
    ) -> Result<Option<customers::Model>, DomainError<CustomerErrorKind>>;

    /// Retrieves all [`Customers`][Vec<customers::Model>].
    async fn get_all<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>>;

    /// Retrieves all [`Customers`][Vec<customers::Model>] with the provided source.
    async fn get_all_by_source<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &str,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>>;

    /// Inserts many [`Customers`][Vec<customers::ActiveModel>].
    async fn insert_many<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        models: Vec<customers::ActiveModel>,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>>;
}
