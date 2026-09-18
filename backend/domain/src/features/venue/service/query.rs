use sea_orm::ConnectionTrait;

use crate::{
    features::venue::{
        error::VenueErrorKind, mapper::VenueMapper, model::Venue, repository::VenueRepository,
    },
    shared::error::DomainError,
};

/// Represents a query service dealing with [`Venues`][Venue].
#[derive(Clone)]
pub struct VenueQueryService<R: VenueRepository> {
    repository: R,
}

impl<R: VenueRepository> VenueQueryService<R> {
    /// Creates a new [`VenueQueryService`].
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Finds a [`Venue`] with the provided ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        db_connection: &C,
        id: &i32,
    ) -> Result<Option<Venue>, DomainError<VenueErrorKind>> {
        self.repository
            .find_by_id(db_connection, id)
            .await
            .map_err(|error| DomainError::from(VenueErrorKind::FindById).with_cause(error))?
            .map(|model| VenueMapper::to_domain_model(&model))
            .transpose()
    }
}
