use domain::{
    features::{
        customer::{error::CustomerErrorKind, repository::CustomerRepository},
        venue::repository::VenueRepository,
    },
    shared::{Source, error::DomainError},
};
use entity::{customers, venues};
use sea_orm::ConnectionTrait;
use square_api_client::features::{
    customer::{CustomerEndpoint, mapper::SquareCustomerMapper, model::Customer},
    location::{LocationEndpoint, mapper::SquareLocationMapper, model::Location},
};

use crate::backfill::error::BackfillErrorKind;

/// Represents a service for backfills from the Square API.
pub struct SquareApiBackfillService<VR: VenueRepository, CR: CustomerRepository> {
    venue_repository: VR,
    customer_repository: CR,
    square_location_endpoint: LocationEndpoint,
    square_customer_endpoint: CustomerEndpoint,
}

impl<VR: VenueRepository, CR: CustomerRepository> SquareApiBackfillService<VR, CR> {
    /// Creates a new [`SquareApiBackfillService`].
    pub fn new(
        venue_repository: VR,
        customer_repository: CR,
        square_location_endpoint: LocationEndpoint,
        square_customer_endpoint: CustomerEndpoint,
    ) -> Self {
        Self {
            venue_repository,
            customer_repository,
            square_location_endpoint,
            square_customer_endpoint,
        }
    }

    /// Loads the main Location from the Square API.
    async fn load_main_location(&self) -> Result<Option<Location>, DomainError<BackfillErrorKind>> {
        let retrieve_main_location_response = self
            .square_location_endpoint
            .retrieve_location("main")
            .await
            .map_err(|error| {
                DomainError::from(BackfillErrorKind::Run)
                    .with_cause(error)
                    .with_context("source", Source::Square.to_string())
                    .with_context("topic", String::from("locations"))
            })?;

        if retrieve_main_location_response.location.is_some() {
            println!("📥 Retrieved the main Square Location");
        }

        Ok(retrieve_main_location_response.location)
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
                    .with_context("source", Source::Square.to_string())
                    .with_context("topic", String::from("customers"))
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
                        .with_context("source", Source::Square.to_string())
                        .with_context("topic", String::from("customers"))
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

    /// Imports the [`Venue`][Optional<venues::ActiveModel>] based on the main Square Location.
    async fn import_venue<C: ConnectionTrait>(
        &self,
        db_connection: &C,
        model_to_import: venues::ActiveModel,
    ) -> Result<(), DomainError<BackfillErrorKind>> {
        self.venue_repository
            .insert(db_connection, model_to_import)
            .await
            .map_err(|error| DomainError::from(BackfillErrorKind::Run).with_cause(error))?;

        println!("✨ Imported a new Venue");

        Ok(())
    }

    /// Imports new [`Square Customers`][Vec<customers::ActiveModel>].
    async fn import_customers<C: ConnectionTrait>(
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

        let square_api_main_location = self.load_main_location().await?;
        let square_api_customers = self.load_all_customers().await?;

        if square_api_customers.is_empty() && square_api_main_location.is_none() {
            return Ok(());
        }

        if let Some(main_square_location) = square_api_main_location {
            let existing_venue = self
                .venue_repository
                .find_by_source_and_source_id(
                    db_connection,
                    &Source::Square.to_string(),
                    &main_square_location.id,
                )
                .await
                .map_err(|error| {
                    DomainError::from(BackfillErrorKind::Run)
                        .with_cause(error)
                        .with_context("source", Source::Square.to_string())
                        .with_context("topic", String::from("venues"))
                })?;

            if existing_venue.is_none() {
                let venue_model_to_import = SquareLocationMapper::to_insert_active_model(
                    &main_square_location,
                )
                .map_err(|error| DomainError::from(BackfillErrorKind::Run).with_cause(error))?;

                self.import_venue(db_connection, venue_model_to_import)
                    .await?;
            } else {
                println!("✅ Already up to date - No new Venue to import.");
            }
        }

        let existing_customer_reference_ids: Vec<String> = self
            .customer_repository
            .get_all_by_source(db_connection, &Source::Square.to_string())
            .await
            .map_err(|error| {
                DomainError::from(BackfillErrorKind::Run)
                    .with_cause(error)
                    .with_context("source", Source::Square.to_string())
                    .with_context("topic", String::from("customers"))
            })?
            .into_iter()
            .filter_map(|model| model.source_id)
            .collect();

        let customer_models_to_import: Vec<customers::ActiveModel> = square_api_customers
            .clone()
            .iter()
            .filter(|customer| !existing_customer_reference_ids.contains(&customer.id))
            .map(SquareCustomerMapper::to_insert_active_model)
            .collect::<Result<Vec<customers::ActiveModel>, DomainError<CustomerErrorKind>>>()
            .map_err(|error| DomainError::from(BackfillErrorKind::Run).with_cause(error))?;

        self.import_customers(db_connection, customer_models_to_import)
            .await?;

        Ok(())
    }
}
