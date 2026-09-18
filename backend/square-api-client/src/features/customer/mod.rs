use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    features::customer::{error::CustomerEndpointErrorKind, model::Customer},
    shared::{
        client::{SquareApiClient, error::SquareClientError},
        response::SquareApiResponseError,
    },
};

pub mod error;
pub mod mapper;
pub mod model;

/// Response envelope for Square's list customers endpoint.
#[derive(Deserialize, Clone)]
pub struct ListCustomersResponse {
    /// The errors in the Square API response if an error occurred.
    pub errors: Option<Vec<SquareApiResponseError>>,
    /// The Square API Customer list if no error occurred.
    pub customers: Option<Vec<Customer>>,
    /// The cursor string in case there are still Customers to load.
    pub cursor: Option<String>,
    /// The total count of Customers.
    pub count: Option<i64>,
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
    /// * [Square API reference](https://developer.squareup.com/reference/square/customers-api/list-customers)
    pub async fn list_customer(
        &self,
        cursor: Option<String>,
    ) -> Result<ListCustomersResponse, SquareClientError<CustomerEndpointErrorKind>> {
        let mut query: HashMap<String, String> =
            HashMap::from([(String::from("count"), true.to_string())]);

        if let Some(cursor) = cursor {
            query.insert(String::from("cursor"), cursor);
        }

        self.client
            .get::<ListCustomersResponse>("/customers", Some(query))
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
                fs::read_to_string("tests/fixtures/customer/customer_list.json").unwrap();

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
                .list_customer(None)
                .await
                .expect("Expected the Customer list to be retrieved successfully.");

            // Then
            assert!(!response.customers.unwrap_or_default().is_empty());
            assert!(response.cursor.is_none());
            assert_eq!(response.count, Some(2));
        }

        #[tokio::test]
        async fn test_list_customer_returns_a_customer_list_with_a_cursor() {
            // Given
            let mock_server = MockServer::start().await;
            let customer_list_fixture =
                fs::read_to_string("tests/fixtures/customer/customer_list_with_cursor.json")
                    .unwrap();

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
                .list_customer(None)
                .await
                .expect("Expected the Customer list to be retrieved successfully.");

            // Then
            assert!(!response.customers.unwrap_or_default().is_empty());
            assert!(response.cursor.is_some());
            assert_eq!(response.count, Some(2));
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
            let response = CustomerEndpoint::new(client).list_customer(None).await;

            // Then
            assert!(response.is_err());
        }
    }
}
