use async_trait::async_trait;
use domain::{
    features::customer::{error::CustomerErrorKind, repository::CustomerRepository},
    shared::error::DomainError,
};
use entity::customers;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

/// Represents a Database [CustomerRepository].
#[derive(Default, Clone)]
pub struct CustomerDatabaseRepository;

#[async_trait]
impl CustomerRepository for CustomerDatabaseRepository {
    async fn find_by_id<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        id: &i32,
    ) -> Result<Option<customers::Model>, DomainError<CustomerErrorKind>> {
        customers::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::FindById).with_cause(error))
    }

    async fn get_all<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
    ) -> Result<Vec<customers::Model>, DomainError<CustomerErrorKind>> {
        customers::Entity::find()
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(CustomerErrorKind::GetAll).with_cause(error))
    }

    async fn get_all_by_source<C: ConnectionTrait + Sync>(
        &self,
        db_connection: &C,
        source: &str,
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
}

#[cfg(test)]
mod tests {

    mod find_by_id {
        use chrono::Utc;
        use domain::features::customer::error::CustomerErrorKind;
        use entity::customers;
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase};

        use crate::features::customer::repository::{
            CustomerDatabaseRepository, CustomerRepository,
        };

        #[tokio::test]
        async fn test_find_by_id_returns_a_matching_customer() {
            // Given
            let expected_found_customer = customers::Model {
                id: 6666,
                first_name: Some("John".to_string()),
                last_name: Some("Osbourne".to_string()),
                nickname: Some("Ozzy".to_string()),
                email_address: Some("ozzy@in.heaven".to_string()),
                date_of_birth: None,
                source: Some("Square".to_string()),
                source_id: Some("CUST123".to_string()),
                created_at: Utc::now().naive_utc(),
                last_updated_at: None,
            };

            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![expected_found_customer.clone()]])
                .into_connection();

            let repository = CustomerDatabaseRepository;

            // When
            let result = repository
                .find_by_id(&db_connection, &6666)
                .await
                .expect("Expected to find a Customer with the provided ID.");

            // Then
            assert_eq!(result, Some(expected_found_customer));
        }

        #[tokio::test]
        async fn test_find_by_id_returns_a_domain_error_when_the_query_fails() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_errors([DbErr::Custom("Sometimes it just doesn't work.".to_string())])
                .into_connection();

            let repository = CustomerDatabaseRepository;

            // When
            let result = repository.find_by_id(&db_connection, &6666).await;

            // Then
            let error = result.expect_err("Expected the query to fail.");
            assert_eq!(error.kind(), &CustomerErrorKind::FindById);
        }
    }

    mod get_all_by_source {
        use chrono::Utc;
        use domain::features::customer::error::CustomerErrorKind;
        use entity::customers;
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase};

        use crate::features::customer::repository::{
            CustomerDatabaseRepository, CustomerRepository,
        };

        #[tokio::test]
        async fn test_get_all_by_source_returns_matching_customers() {
            // Given
            let expected_found_customer = customers::Model {
                id: 6666,
                first_name: Some("John".to_string()),
                last_name: Some("Osbourne".to_string()),
                nickname: Some("Ozzy".to_string()),
                email_address: Some("ozzy@in.heaven".to_string()),
                date_of_birth: None,
                source: Some("Square".to_string()),
                source_id: Some("CUST123".to_string()),
                created_at: Utc::now().naive_utc(),
                last_updated_at: None,
            };

            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![expected_found_customer.clone()]])
                .into_connection();

            let repository = CustomerDatabaseRepository;

            // When
            let result = repository
                .get_all_by_source(&db_connection, "Square")
                .await
                .expect("Expected to get all Customers by Source.");

            // Then
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], expected_found_customer);
        }

        #[tokio::test]
        async fn test_get_all_by_source_returns_a_domain_error_when_the_query_fails() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_errors([DbErr::Custom("Sometimes it just doesn't work.".to_string())])
                .into_connection();

            let repository = CustomerDatabaseRepository;

            // When
            let result = repository.get_all_by_source(&db_connection, "Square").await;

            // Then
            let error = result.expect_err("Expected the query to fail.");
            assert_eq!(error.kind(), &CustomerErrorKind::GetAllBySource);
        }
    }

    mod insert_many {
        use chrono::Utc;
        use domain::features::customer::error::CustomerErrorKind;
        use entity::customers;
        use sea_orm::{ActiveValue::Set, DatabaseBackend, DbErr, MockDatabase};

        use crate::features::customer::repository::{
            CustomerDatabaseRepository, CustomerRepository,
        };

        #[tokio::test]
        async fn test_insert_many_returns_inserted_customers() {
            // Given
            let expected_inserted_customer = customers::Model {
                id: 6666,
                first_name: Some("John".to_string()),
                last_name: Some("Osbourne".to_string()),
                nickname: Some("Ozzy".to_string()),
                email_address: Some("ozzy@in.heaven".to_string()),
                date_of_birth: None,
                source: Some("Square".to_string()),
                source_id: Some("CUST123".to_string()),
                created_at: Utc::now().naive_utc(),
                last_updated_at: None,
            };

            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([vec![expected_inserted_customer.clone()]])
                .into_connection();

            let repository = CustomerDatabaseRepository;
            let models_to_insert = vec![customers::ActiveModel {
                nickname: Set(Some("Ozzy".to_string())),
                ..Default::default()
            }];

            // When
            let result = repository
                .insert_many(&db_connection, models_to_insert)
                .await
                .expect("Expected insert to succeed.");

            // Then
            assert_eq!(result, vec![expected_inserted_customer]);
        }

        #[tokio::test]
        async fn test_insert_many_returns_empty_vec_without_querying_when_given_no_models() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
            let repository = CustomerDatabaseRepository;

            // When
            let result = repository
                .insert_many(&db_connection, vec![])
                .await
                .expect("Expected the insertion to be successful.");

            // Then
            assert!(result.is_empty());
        }

        #[tokio::test]
        async fn test_insert_many_returns_a_domain_error_when_the_query_fails() {
            // Given
            let db_connection = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_errors([DbErr::Custom("Sometimes it just doesn't work.".to_string())])
                .into_connection();

            let repository = CustomerDatabaseRepository;

            // When
            let result = repository
                .insert_many(
                    &db_connection,
                    vec![customers::ActiveModel {
                        nickname: Set(Some("Ozzy".to_string())),
                        ..Default::default()
                    }],
                )
                .await;

            // Then
            let error = result.expect_err("Expected the query to fail.");
            assert_eq!(error.kind(), &CustomerErrorKind::InsertMany);
        }
    }
}
