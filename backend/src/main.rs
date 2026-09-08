use api::{Api, shared::error::StartupError};

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    Api::setup().serve(None).await
}
