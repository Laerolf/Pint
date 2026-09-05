use async_trait::async_trait;
use domain::{features::customer::error::CustomerErrorKind, shared::error::DomainError};
use entity::customers;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

/// Represents a repository dealing with [`Customers`][customers::Model].
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// Retrieves all [`Customers`][Vec<customers::Model>] with the provided source.
    async fn get_all_by_source<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &String,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>>;

    /// Inserts many [`Customers`][Vec<customers::ActiveModel>].
    async fn insert_many<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        models: Vec<customers::ActiveModel>,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>>;

    /// Updates a [`Customer`]customers::ActiveModel].
    async fn update<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        model: customers::ActiveModel,
    ) -> Result<customers::Model, DomainError<CustomerErrorKind>>;
}

/// Represents a Database [CustomerRepository].
#[derive(Default)]
pub struct CustomerDatabaseRepository;

#[async_trait]
impl CustomerRepository for CustomerDatabaseRepository {
    async fn get_all_by_source<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &String,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>> {
        customers::Entity::find()
            .filter(customers::Column::Source.eq(source))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::GetAllBySource).with_cause(error))
    }

    async fn insert_many<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        models: Vec<customers::ActiveModel>,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>> {
        if models.is_empty() {
            return Ok(Vec::default());
        }

        customers::Entity::insert_many(models)
            .exec_with_returning(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::InsertMany).with_cause(error))
    }

    async fn update<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        model: customers::ActiveModel,
    ) -> Result<customers::Model, DomainError<CustomerErrorKind>> {
        customers::Entity::update(model)
            .exec(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::Update).with_cause(error))
    }
}
