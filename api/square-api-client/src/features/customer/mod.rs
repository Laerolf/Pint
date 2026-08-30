use serde::Deserialize;

use crate::{
    features::customer::{error::CustomerEndpointErrorKind, model::Customer},
    shared::{
        client::{SquareApiClient, error::SquareClientError},
        response::SquareApiResponseError,
    },
};

pub mod error;
pub mod model;

/// Response envelope for Square's list customers endpoint.
#[derive(Deserialize)]
pub struct ListCustomersResponse {
    pub errors: Option<Vec<SquareApiResponseError>>,
    pub customers: Option<Vec<Customer>>,
    pub cursor: Option<String>,
}

/// Represents the available Square Customers API endpoints.
pub struct CustomerEndpoint {
    client: SquareApiClient,
}

impl CustomerEndpoint {
    /// Creates a new [`CustomerEndpoint`].
    pub fn new(client: SquareApiClient) -> Self {
        Self { client }
    }

    /// Retrieves a list of all existing Customers in the Square API.
    pub async fn list_customer(
        &self,
    ) -> Result<ListCustomersResponse, SquareClientError<CustomerEndpointErrorKind>> {
        self.client
            .get::<ListCustomersResponse>("/customers", None)
            .await
            .map_err(|error| {
                SquareClientError::from(CustomerEndpointErrorKind::ListCustomer).with_cause(error)
            })
    }
}

#[cfg(test)]
mod tests {
    mod list_customer {
        use std::fs;
        use wiremock::{
            Mock, MockServer, ResponseTemplate,
            matchers::{method, path},
        };

        use crate::{
            features::customer::CustomerEndpoint,
            shared::client::{SquareApiClient, SquareApiClientOptions},
        };

        #[tokio::test]
        async fn test_list_customer_returns_a_customer_list() {
            // Given
            let mock_server = MockServer::start().await;
            let customer_list_fixture =
                fs::read_to_string("tests/fixtures/customer_list.json").unwrap();

            Mock::given(method("GET"))
                .and(path("/customers"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(customer_list_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = CustomerEndpoint::new(client)
                .list_customer()
                .await
                .expect("Expected the Customer list to be retrieved successfully.");

            // Then
            assert_eq!(response.customers.unwrap_or_default().len(), 1);
            assert!(response.cursor.is_none());
        }

        #[tokio::test]
        async fn test_list_customer_returns_a_customer_list_with_a_cursor() {
            // Given
            let mock_server = MockServer::start().await;
            let customer_list_fixture =
                fs::read_to_string("tests/fixtures/customer_list_with_cursor.json").unwrap();

            Mock::given(method("GET"))
                .and(path("/customers"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(customer_list_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = CustomerEndpoint::new(client)
                .list_customer()
                .await
                .expect("Expected the Customer list to be retrieved successfully.");

            // Then
            assert_eq!(response.customers.unwrap_or_default().len(), 1);
            assert!(response.cursor.is_some());
        }

        #[tokio::test]
        async fn test_list_customer_throws_an_error() {
            // Given
            let mock_server = MockServer::start().await;
            let errors_fixture = fs::read_to_string("tests/fixtures/errors.json").unwrap();

            Mock::given(method("GET"))
                .and(path("/customers"))
                .respond_with(
                    ResponseTemplate::new(429).set_body_raw(errors_fixture, "application/json"),
                )
                .mount(&mock_server)
                .await;

            let square_api_token = "test-test";
            let square_api_version = "2026-08-19";

            let client = SquareApiClient::new(SquareApiClientOptions {
                base_url: mock_server.uri().parse().unwrap(),
                square_api_token: square_api_token.to_string(),
                square_api_version: square_api_version.to_string(),
            })
            .expect("Expected the test Square API client creation to resolve.");

            // When
            let response = CustomerEndpoint::new(client).list_customer().await;

            // Then
            assert!(response.is_err());
        }
    }
}
