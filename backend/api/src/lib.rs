use std::sync::Arc;

use axum::{Json, Router, http::HeaderValue, routing::get, serve};
use domain::features::{
    customer::repository::CustomerRepository, venue::repository::VenueRepository,
};
use infrastructure::features::{
    customer::repository::CustomerDatabaseRepository, location::repository::VenueDatabaseRepository,
};
use migration::{Migrator, MigratorTrait};
use reqwest::{
    Method, Url,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, TransactionTrait};
use square_api_client::{
    features::{customer::CustomerEndpoint, location::LocationEndpoint},
    shared::client::{SquareApiClient, SquareApiClientOptions},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use utoipa::openapi::OpenApi;
use utoipa_scalar_warpper::{Config, Scalar};

use crate::{
    backfill::square::SquareApiBackfillService,
    features::openapi,
    shared::{context::ApiContext, environment::Environment, error::StartupError},
};

pub mod backfill;
pub mod features;
pub mod shared;

/// Returns the OpenAPI documentation.
async fn openapi_json() -> Json<OpenApi> {
    Json(openapi())
}

/// Represents the API.
pub struct Api;

impl Api {
    /// Creates a new [`Api`].
    pub fn setup() -> Self {
        Self {}
    }

    /// Returns a new [`DatabaseConnection`].
    async fn open_database_connection(
        &self,
        database_url: &String,
    ) -> Result<DatabaseConnection, StartupError> {
        let opt = ConnectOptions::new(database_url);

        Database::connect(opt)
            .await
            .map_err(|_error| StartupError::InvalidDbUrl)
    }

    /// Runs a [`Square API backfill`][SquareApiBackfillService].
    async fn run_square_api_backfill<
        C: ConnectionTrait,
        VR: VenueRepository,
        CR: CustomerRepository,
    >(
        &self,
        square_api_base_url: &str,
        square_api_token: &String,
        square_api_version: &String,
        venue_repository: VR,
        customer_repository: CR,
        db_connection: &C,
    ) -> Result<(), StartupError> {
        let square_api_client = SquareApiClient::new(SquareApiClientOptions {
            base_url: Url::parse(square_api_base_url)
                .expect("Expected the Square API base url to resolve successfully."),
            square_api_token: square_api_token.clone(),
            square_api_version: square_api_version.clone(),
        })
        .map_err(|_error| StartupError::CreateSquareApiClient)?;

        let square_location_endpoint = LocationEndpoint::new(square_api_client.clone());
        let square_customer_endpoint = CustomerEndpoint::new(square_api_client);

        SquareApiBackfillService::new(
            venue_repository,
            customer_repository,
            square_location_endpoint,
            square_customer_endpoint,
        )
        .run(db_connection)
        .await
        .map_err(|_error| StartupError::CreateSquareApiClient)
    }

    /// Creates a new [`Router`].
    pub fn create_router(
        &self,
        context: ApiContext<
            DatabaseConnection,
            VenueDatabaseRepository,
            CustomerDatabaseRepository,
        >,
        cors_allow_origin: &String,
    ) -> Router {
        // TODO: Adjust accordingly
        let cors = CorsLayer::new()
            .allow_origin(cors_allow_origin.parse::<HeaderValue>().unwrap())
            .allow_credentials(true)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let scalar_config = Config::default().theme("saturn").hide_models(true);

        Router::new()
            .merge(
                Scalar::new(openapi())
                    .with_url("/scalar")
                    .with_config(scalar_config),
            )
            .route("/openapi.json", get(openapi_json))
            .nest("/api", features::routes())
            .layer(cors)
            .with_state(context)
    }

    /// Sets up the Api and start serving.
    pub async fn serve(&self, environment_file_name: Option<String>) -> Result<(), StartupError> {
        let environment = Environment::from(environment_file_name.unwrap_or(String::from(".env")));

        let db_connection = self
            .open_database_connection(environment.database_url())
            .await?;

        Migrator::up(&db_connection, None)
            .await
            .map_err(|_error| StartupError::DatabaseMigration)?;

        let square_api_backfill_db_transaction = db_connection
            .begin()
            .await
            .map_err(|_error| StartupError::RunSquareApiBackfill)?;

        let square_api_backfill_result = self
            .run_square_api_backfill(
                environment.square_api_base_url(),
                environment.square_api_token(),
                environment.square_api_version(),
                VenueDatabaseRepository,
                CustomerDatabaseRepository,
                &square_api_backfill_db_transaction,
            )
            .await;

        match square_api_backfill_result {
            Ok(_value) => square_api_backfill_db_transaction
                .commit()
                .await
                .map_err(|_error| StartupError::RunSquareApiBackfill)?,
            Err(_error) => square_api_backfill_db_transaction
                .rollback()
                .await
                .map_err(|_error| StartupError::RunSquareApiBackfill)?,
        }

        let context = ApiContext::new(
            Arc::new(db_connection),
            VenueDatabaseRepository,
            CustomerDatabaseRepository,
        );

        let router = self.create_router(context, environment.cors_allow_origin());

        let host_url = format!("{}:{}", environment.host(), environment.port());

        let listener = TcpListener::bind(host_url)
            .await
            .expect("Expected to create an API listener.");

        if let Ok(address) = listener.local_addr() {
            println!("🌐 The Pint API is listening on http://{:?}", address);
            println!("📄 Scalar is available on http://{:?}/scalar", address);
        }

        serve(listener, router)
            .await
            .map_err(|_error| StartupError::Serve)
    }
}
