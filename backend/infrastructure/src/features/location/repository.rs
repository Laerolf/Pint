use async_trait::async_trait;
use domain::{
    features::venue::{error::VenueErrorKind, repository::VenueRepository},
    shared::error::DomainError,
};
use entity::venues;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

/// Represents a Database [VenueRepository].
#[derive(Default, Clone)]
pub struct VenueDatabaseRepository;

#[async_trait]
impl VenueRepository for VenueDatabaseRepository {
    async fn find_by_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        id: &i32,
    ) -> Result<Option<venues::Model>, DomainError<VenueErrorKind>> {
        venues::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(VenueErrorKind::FindById)
                    .with_cause(error)
                    .with_context("id", id.to_string())
            })
    }

    async fn find_by_source_and_source_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &str,
        source_id: &str,
    ) -> Result<Option<venues::Model>, DomainError<VenueErrorKind>> {
        venues::Entity::find()
            .filter(venues::Column::Source.eq(source))
            .filter(venues::Column::SourceId.eq(source_id))
            .one(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(VenueErrorKind::FindBySourceAndSourceId)
                    .with_cause(error)
                    .with_context("source", source)
                    .with_context("source_id", source_id)
            })
    }

    async fn insert<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        model: venues::ActiveModel,
    ) -> Result<venues::Model, DomainError<VenueErrorKind>> {
        model
            .insert(db_connection)
            .await
            .map_err(|error| DomainError::from(VenueErrorKind::Insert).with_cause(error))
    }
}
