#[cfg(test)]
pub mod tests {
    use std::env;

    use reqwest::Url;
    use square_api_client::{
        features::customer::CustomerEndpoint,
        shared::client::{SquareApiClient, SquareApiClientOptions},
    };

    #[tokio::test]
    async fn test_retrieve_customer_list_from_square_api() {
        // Given
        dotenvy::from_filename(".env.test").ok();

        let square_api_version = env::var("SQUARE_API_VERSION")
            .expect("Expected to have a SQUARE_API_VERSION test environment variable.");
        let square_api_base_url = env::var("SQUARE_API_BASE_URL")
            .expect("Expected to have a SQUARE_API_BASE_URL test environment variable.");
        let square_api_token = env::var("SQUARE_API_TOKEN")
            .expect("Expected to have a SQUARE_API_TOKEN test environment variable.");

        let square_api_client = SquareApiClient::new(SquareApiClientOptions {
            base_url: Url::parse(&square_api_base_url)
                .expect("Expected the test Square API base URL to resolve."),
            square_api_token,
            square_api_version,
        })
        .expect("Expected the test Square API client to resolve.");

        let customer_api_endpoint = CustomerEndpoint::new(square_api_client);

        // When
        let response = customer_api_endpoint
            .list_customer()
            .await
            .expect("Expected to retrieve the Customer list successfully.");

        // Then
        assert_eq!(response.customers.unwrap_or_default().len(), 2);
    }
}
