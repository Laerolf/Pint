use std::env;

/// Represents all relevant environment variables.
pub struct Environment {
    /// The host to run the API on.
    host: String,
    /// The port to run the API on.
    port: String,

    /// The address to allow to reach out to the API.
    cors_allow_origin: String,

    /// The database URL to connect to.
    database_url: String,

    /// The Square API version.
    square_api_version: String,
    /// The Square API base url.
    square_api_base_url: String,
    /// The Square API access token.
    square_api_token: String,
}

impl Environment {
    /// Creates a new [`Environment`] based on the provided file name content.
    pub fn from(file_name: impl Into<String>) -> Self {
        dotenvy::from_filename(file_name.into()).ok();

        let host = env::var("HOST").expect("Expected to have a HOST test environment variable.");
        let port = env::var("PORT").expect("Expected to have a PORT test environment variable.");

        let cors_allow_origin = env::var("CORS_ALLOW_ORIGIN")
            .expect("Expected to have a CORS_ALLOW_ORIGIN test environment variable.");

        let database_url = env::var("DATABASE_URL")
            .expect("Expected to have a DATABASE_URL test environment variable.");

        let square_api_version = env::var("SQUARE_API_VERSION")
            .expect("Expected to have a SQUARE_API_VERSION test environment variable.");
        let square_api_base_url = env::var("SQUARE_API_BASE_URL")
            .expect("Expected to have a SQUARE_API_BASE_URL test environment variable.");
        let square_api_token = env::var("SQUARE_API_TOKEN")
            .expect("Expected to have a SQUARE_API_TOKEN test environment variable.");

        Self {
            host,
            port,
            cors_allow_origin,
            database_url,
            square_api_version,
            square_api_base_url,
            square_api_token,
        }
    }

    /// Returns the host to run the API on.
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Returns the port to run the API on.
    pub fn port(&self) -> &String {
        &self.port
    }

    /// Returns the address to allow to reach out to the API.
    pub fn cors_allow_origin(&self) -> &String {
        &self.cors_allow_origin
    }

    /// Returns the database URL to connect to.
    pub fn database_url(&self) -> &String {
        &self.database_url
    }

    /// Returns the Square API version.
    pub fn square_api_version(&self) -> &String {
        &self.square_api_version
    }

    /// Returns the Square API base url.
    pub fn square_api_base_url(&self) -> &String {
        &self.square_api_base_url
    }

    /// Returns the Square API access token.
    pub fn square_api_token(&self) -> &String {
        &self.square_api_token
    }
}
