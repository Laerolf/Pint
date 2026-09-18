use async_trait::async_trait;
use entity::venues;
use sea_orm::ConnectionTrait;

use crate::{features::venue::error::VenueErrorKind, shared::error::DomainError};

/// Represents a repository dealing with [`Venues`][venues::Model].
#[async_trait]
pub trait VenueRepository: Send + Sync {
    /// Finds a [`Venue`][venues::Model] with the provided ID.
    async fn find_by_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        id: &i32,
    ) -> Result<Option<venues::Model>, DomainError<VenueErrorKind>>;

    /// Finds a [`Venue`][venues::Model] with the provided source and source ID.
    async fn find_by_source_and_source_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &str,
        source_id: &str,
    ) -> Result<Option<venues::Model>, DomainError<VenueErrorKind>>;

    /// Inserts many [`Venues`][Vec<venues::ActiveModel>].
    async fn insert<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        model: venues::ActiveModel,
    ) -> Result<venues::Model, DomainError<VenueErrorKind>>;
}
