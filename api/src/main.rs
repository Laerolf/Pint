use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    serve,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::shared::{context::Context, environment::Environment};

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
