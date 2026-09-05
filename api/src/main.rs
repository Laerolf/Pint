use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    serve,
};
use infrastructure::features::customer::repository::CustomerDatabaseRepository;
use migration::{Migrator, MigratorTrait};
use reqwest::Url;
use sea_orm::{ConnectOptions, Database};
use square_api_client::{
    features::customer::CustomerEndpoint,
    shared::client::{SquareApiClient, SquareApiClientOptions},
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::{
    backfill::square::SquareApiBackfillService,
    shared::{context::Context, environment::Environment},
};

pub mod backfill;
pub mod features;
pub mod shared;

#[tokio::main]
async fn main() {
    let environment = Environment::from(".env");

    let opt = ConnectOptions::new(environment.database_url());

    let db_connection = Database::connect(opt)
        .await
        .expect("Failed to create a database connection.");

    Migrator::up(&db_connection, None)
        .await
        .expect("Expected the database migrations to run successfully.");

    let customer_repository = CustomerDatabaseRepository::default();

    let square_api_client = SquareApiClient::new(SquareApiClientOptions {
        base_url: Url::parse(environment.square_api_base_url())
            .expect("Expected the Square API base url to resolve successfully."),
        square_api_token: environment.square_api_token().clone(),
        square_api_version: environment.square_api_version().clone(),
    })
    .expect("Expected for the Square API client to resolve successfully.");

    let square_customer_endpoint = CustomerEndpoint::new(square_api_client);

    SquareApiBackfillService::new(customer_repository, square_customer_endpoint)
        .run(&db_connection)
        .await
        .expect("Expected the Square API backfill to resolve successfully.");

    // TODO: Adjust accordingly
    let cors = CorsLayer::new()
        .allow_origin(
            environment
                .cors_allow_origin()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .layer(cors)
        .with_state(Context::new(Arc::new(db_connection)));

    let host_url = format!("{}:{}", environment.host(), environment.port());

    let listener = TcpListener::bind(host_url)
        .await
        .expect("Expected to create an API listener.");

    if let Ok(address) = listener.local_addr() {
        println!("🌐 The Tao API is listening on http://{:?}", address);
    }

    serve(listener, router).await.unwrap();
}
