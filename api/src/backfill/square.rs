use domain::{
    features::customer::error::CustomerErrorKind,
    shared::{Source, error::DomainError},
};
use entity::customers;
use infrastructure::features::customer::repository::{
    CustomerDatabaseRepository, CustomerRepository,
};
use sea_orm::ConnectionTrait;
use square_api_client::features::customer::{CustomerEndpoint, model::Customer};

use crate::{
    backfill::error::BackfillErrorKind, features::customers::mapper::SquareCustomerMapper,
};

/// Represents a service for backfills from the Square API.
pub struct SquareApiBackfillService {
    customer_repository: CustomerDatabaseRepository,
    square_customer_endpoint: CustomerEndpoint,
}

impl SquareApiBackfillService {
    /// Creates a new [`SquareApiBackfillService`].
    pub fn new(
        customer_repository: CustomerDatabaseRepository,
        square_customer_endpoint: CustomerEndpoint,
    ) -> Self {
        Self {
            customer_repository,
            square_customer_endpoint,
        }
    }

    /// Loads all Customers from the Square API.
    async fn load_all_customers(&self) -> Result<Vec<Customer>, DomainError<BackfillErrorKind>> {
        let mut square_api_response = self
            .square_customer_endpoint
            .list_customer(None)
            .await
            .map_err(|error| {
                DomainError::from(BackfillErrorKind::Run)
                    .with_cause(error)
                    .with_context("source", &Source::Square.to_string())
            })?;

        let count = square_api_response.count.unwrap_or_default();

        println!(
            "😁 Found {:?} Customer{} in the Square API!",
            count,
            if count == 1 { "" } else { "s" }
        );

        let mut all_customers = square_api_response.customers.unwrap_or_default();

        while let Some(cursor) = square_api_response.cursor {
            square_api_response = self
                .square_customer_endpoint
                .list_customer(Some(cursor))
                .await
                .map_err(|error| {
                    DomainError::from(BackfillErrorKind::Run)
                        .with_cause(error)
                        .with_context("source", &Source::Square.to_string())
                })?;

            all_customers.extend(square_api_response.customers.unwrap_or_default());
        }

        println!(
            "📥 Retrieved all {} Square Customer{}",
            all_customers.len(),
            if all_customers.len() == 1 { "" } else { "s" }
        );

        Ok(all_customers)
    }

    /// Imports new [`Square Customers`][Vec<customers::ActiveModel>].
    async fn import_users<C: ConnectionTrait>(
        &self,
        db_connection: &C,
        models_to_import: Vec<customers::ActiveModel>,
    ) -> Result<(), DomainError<BackfillErrorKind>> {
        let imported_models = self
            .customer_repository
            .insert_many(db_connection, models_to_import)
            .await
            .map_err(|error| DomainError::from(BackfillErrorKind::Run).with_cause(error))?;

        if imported_models.is_empty() {
            println!("✅ Already up to date - No new Customers to import.");
        } else {
            println!(
                "✨ Imported {} new Customer{}",
                imported_models.len(),
                if imported_models.len() == 1 { "" } else { "s" }
            );
        }

        Ok(())
    }

    /// Runs the [`SquareApiBackfill`].
    pub async fn run<C: ConnectionTrait>(
        &self,
        db_connection: &C,
    ) -> Result<(), DomainError<BackfillErrorKind>> {
        println!("🔃 Running the Square API backfill...");

        let square_api_customers = self.load_all_customers().await?;

        if square_api_customers.is_empty() {
            return Ok(());
        }

        let existing_customer_reference_ids: Vec<String> = self
            .customer_repository
            .get_all_by_source(db_connection, &Source::Square.to_string())
            .await
            .map_err(|error| {
                DomainError::from(BackfillErrorKind::Run)
                    .with_cause(error)
                    .with_context("source", &Source::Square.to_string())
            })?
            .into_iter()
            .filter_map(|model| model.source_id)
            .collect();

        let models_to_import: Vec<customers::ActiveModel> = square_api_customers
            .clone()
            .into_iter()
            .filter(|customer| !existing_customer_reference_ids.contains(&customer.id))
            .map(SquareCustomerMapper::to_insert_active_model)
            .collect::<Result<Vec<customers::ActiveModel>, DomainError<CustomerErrorKind>>>()
            .map_err(|error| DomainError::from(BackfillErrorKind::Run).with_cause(error))?;

        self.import_users(db_connection, models_to_import).await?;

        Ok(())
    }
}
